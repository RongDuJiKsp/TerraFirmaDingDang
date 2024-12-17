use crate::tf_serde::operator::{TFConditionOp, TFOperator};
use crate::tf_serde::search_stack::SearchStack;
use std::collections::VecDeque;
use strum::IntoEnumIterator;
const SEARCH_RANGE: i32 = 128;

#[derive(Clone)]
struct SearchState {
    location: i32,
    stack: SearchStack,
}
pub struct SearchSolver {
    condition: [TFConditionOp; 3],
}
impl SearchSolver {
    pub fn search_solve(&self, start_location: i32) -> Vec<TFOperator> {
        let mut bfs_que = VecDeque::new();
        bfs_que.push_back(SearchState {
            location: start_location,
            stack: SearchStack::new(self.condition.clone()),
        });
        while let Some(SearchState {
            location: now_location,
            stack: now_stack,
        }) = bfs_que.pop_front()
        {
            for steps in TFOperator::iter() {
                let next_local: i32 = now_location + <TFOperator as Into<i32>>::into(steps.clone());
                if !(-SEARCH_RANGE <= next_local && next_local <= SEARCH_RANGE) {
                    continue; //优化：如果超过打铁可以接受的范围，则放弃这个解
                }
                let mut next_stack = now_stack.clone();
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
