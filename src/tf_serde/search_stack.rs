use crate::tf_serde::operator::{TFConditionOp, TFOperator};
#[derive(Debug, Clone)]
pub struct SearchStack {
    stack: Vec<TFOperator>,
    conditions: [TFConditionOp; 3],
    condition_ok: [bool; 3],
}
