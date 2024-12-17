use crate::tf_serde::operator::{TFConditionOp, TFOperator};
use crate::tf_serde::search_stack::SearchStack;
use std::collections::VecDeque;
use strum::IntoEnumIterator;
pub const SEARCH_RANGE: i32 = 128;

#[derive(Clone)]
struct SearchState {
    location: i32,
    stack: SearchStack,
}
pub struct SearchSolver {
    condition: [TFConditionOp; 3],
}
impl SearchSolver {
    pub fn search_solve(
        &self,
        start_location: i32,
        left_limit: i32,
        right_limit: i32,
    ) -> Vec<TFOperator> {
        let mut bfs_que = VecDeque::new();
        bfs_que.push_back(SearchState {
            location: start_location,
            stack: SearchStack::new(self.condition.clone()),
        });
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
                if next_local == 0 && next_stack.ok() {
                    return next_stack.inner();
                }
                bfs_que.push_back(SearchState {
                    location: next_local,
                    stack: next_stack,
                })
            }
        }
        vec![]
    }
    pub fn with_condition(condition: [TFConditionOp; 3]) -> SearchSolver {
        SearchSolver { condition }
    }
}
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
    true
}
