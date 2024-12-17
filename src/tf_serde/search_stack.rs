use crate::tf_serde::operator::{TFConditionOp, TFOperator};
#[derive(Debug, Clone)]
pub struct SearchStack {
    stack: Vec<TFOperator>,
    conditions: [TFConditionOp; 3],
    condition_ok: [bool; 3],
}
impl SearchStack {
    pub fn push(&mut self, e: TFOperator) {
        self.stack.push(e);
        let len = self.stack.len();
        let size = self.conditions.len() - self.conditions.iter().filter(|x| x.is_none()).count();
        for (idx, c) in self.conditions.iter().enumerate() {
            self.condition_ok[idx] = match c {
                TFConditionOp::Last(o) => len >= 1 && o.eq(&self.stack[len - 1]),
                TFConditionOp::LastSecond(o) => len >= 2 && o.eq(&self.stack[len - 2]),
                TFConditionOp::LastThird(o) => len >= 3 && o.eq(&self.stack[len - 3]),
                TFConditionOp::NotLast(o) => {
                    len > 0
                        && o.ne(&self.stack[len - 1])
                        && len >= size
                        && self.stack[len - size..len - 1]
                            .iter()
                            .filter(|x| o.eq(*x))
                            .count()
                            > 0
                }
                TFConditionOp::Any(o) => {
                    self.condition_ok[idx] || (len >= 1 && o.eq(&self.stack[len - 1]))
                }
                TFConditionOp::None => true,
            };
        }
    }
    pub fn ok(&self) -> bool {
        self.condition_ok[0] && self.condition_ok[1] && self.condition_ok[2]
    }
    pub fn new(cond: [TFConditionOp; 3]) -> SearchStack {
        SearchStack {
            stack: Vec::new(),
            conditions: cond,
            condition_ok: [false; 3],
        }
    }
    pub fn inner(self) -> Vec<TFOperator> {
        self.stack
    }
}
