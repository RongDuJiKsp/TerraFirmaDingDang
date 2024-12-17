use crate::tf_serde::operator::{TFConditionOp, TFOperator};
use crate::tf_serde::search_stack::SearchStack;
use std::collections::VecDeque;
use strum::IntoEnumIterator;

#[derive(Clone)]
struct SearchState {
    location: i32,
    stack: SearchStack,
}
struct SearchSolver {
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
                let next_local: i32 = now_location + steps.clone().into();
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
}
