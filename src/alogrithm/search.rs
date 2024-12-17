use crate::tf_serde::magic_vals::STEP_CONDITION;
use crate::tf_serde::operator::{TFConditionOp, TFOperator};
use crate::tf_serde::search_stack::SearchStack;
use std::collections::{HashSet, VecDeque};
use strum::IntoEnumIterator;

#[derive(Clone)]
struct SearchState {
    location: i32,
    stack: SearchStack,
}
//一个基本的状态摘要
#[derive(Hash, PartialEq, Eq)]
struct SearchZippedState {
    sum_of: i32,
    last_step: [Option<TFOperator>; STEP_CONDITION],
    cond: [bool; STEP_CONDITION],
}
impl SearchZippedState {
    //对状态进行摘要
    //显然两个状态等效当且仅当最后几个取摘要的相同且cond状态相同
    //理论上是小于等于3个 但是取多一个影响不大
    fn from_state(value: &SearchState) -> Self {
        let stk = value.stack.borrow_inner();
        let stk_len = stk.len();
        Self {
            sum_of: stk
                .iter()
                .map(|x| <TFOperator as Into<i32>>::into(*x))
                .sum(),
            last_step: (1..=STEP_CONDITION)
                .rev()
                .map(|x| stk.get(stk_len - x).cloned())
                .collect::<Vec<_>>()
                .try_into()
                .expect("对状态进行摘要时发生错误，请联系管理员"),
            cond: value.stack.conditions().clone(),
        }
    }
}
pub struct SearchSolver {
    condition: [TFConditionOp; STEP_CONDITION],
}
impl SearchSolver {
    pub fn search_solve(
        &self,
        start_location: i32,
        left_limit: i32,
        right_limit: i32,
    ) -> Vec<TFOperator> {
        let mut bfs_que = VecDeque::new();
        let mut vised = HashSet::new();
        let init_state = SearchState {
            location: start_location,
            stack: SearchStack::new(self.condition.clone()),
        };
        vised.insert(SearchZippedState::from_state(&init_state));
        bfs_que.push_back(init_state);
        while let Some(this_state) = bfs_que.pop_front() {
            for steps in TFOperator::iter() {
                let next_local: i32 =
                    this_state.location + <TFOperator as Into<i32>>::into(steps.clone());
                if !should_continue(
                    start_location,
                    &this_state,
                    steps,
                    next_local,
                    left_limit,
                    right_limit,
                ) {
                    continue;
                }
                let mut next_stack = this_state.stack.clone();
                next_stack.push(steps);
                let next_state = SearchState {
                    location: next_local,
                    stack: next_stack,
                };
                let next_state_zipped = SearchZippedState::from_state(&next_state);
                if next_state.location == 0 && next_state.stack.ok() {
                    return next_state.stack.inner();
                }
                if vised.contains(&next_state_zipped) {
                    continue;
                }
                vised.insert(next_state_zipped);
                bfs_que.push_back(next_state);
            }
        }
        vec![]
    }
    pub fn with_condition(condition: [TFConditionOp; STEP_CONDITION]) -> SearchSolver {
        SearchSolver { condition }
    }
}
const FAST_STEP_BY_LIM: i32 = -48; //这个参数为启发式条件--快速步进
fn should_continue(
    start_location: i32,
    this_search_state: &SearchState,
    next_step: TFOperator,
    next_location: i32,
    left_lim: i32,
    right_lim: i32,
) -> bool {
    if !(left_lim <= next_location && next_location <= right_lim) {
        return false; //优化：如果超过打铁可以接受的范围，则放弃这个解
    }
    if start_location < 0 && next_location < start_location {
        return false; //首次操作不允许向右
    }
    if this_search_state.location < FAST_STEP_BY_LIM
        && <TFOperator as Into<i32>>::into(next_step) < 0
    {
        return false; //优化：当当前的位置小于快速步进范围(建议值为2+7+13+16=38) 时 不试图往左走
    }

    true
}
