use crate::tf_serde::search_stack::SearchStack;
use std::collections::VecDeque;

#[derive(Clone)]
struct StarchState {
    location: i32,
    stack: SearchStack,
}
pub fn search_solve(start_location: i32) {
    let mut bfs_que = VecDeque::new();
}
