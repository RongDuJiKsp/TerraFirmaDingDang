use anyhow::anyhow;
use std::error::Error;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TFOperator {
    Tapping,        // 轻击
    Hammering,      // 击打
    HeavyHammering, // 重击
    Drawing,        // 牵拉
    Stamping,       // 冲压
    Bending,        // 弯曲
    Forging,        // 镦锻
    Upsetting,      // 收缩
}
impl Into<i32> for TFOperator {
    fn into(self) -> i32 {
        match self {
            TFOperator::Tapping => -3,
            TFOperator::Hammering => -6,
            TFOperator::Stamping => 2,
            TFOperator::Bending => 7,
            TFOperator::HeavyHammering => -9,
            TFOperator::Drawing => -15,
            TFOperator::Forging => 13,
            TFOperator::Upsetting => 16,
        }
    }
}
impl Into<char> for TFOperator {
    fn into(self) -> char {
        match self {
            TFOperator::Tapping => 'T',
            TFOperator::Hammering => 'H',
            TFOperator::Stamping => 'S',
            TFOperator::Bending => 'B',
            TFOperator::HeavyHammering => 'X',
            TFOperator::Drawing => 'D',
            TFOperator::Forging => 'F',
            TFOperator::Upsetting => 'U',
        }
    }
}
impl TryFrom<char> for TFOperator {
    type Error = Box<dyn Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'T' => Ok(TFOperator::Tapping),
            'H' => Ok(TFOperator::Hammering),
            'S' => Ok(TFOperator::Stamping),
            'B' => Ok(TFOperator::Bending),
            'X' => Ok(TFOperator::HeavyHammering),
            'D' => Ok(TFOperator::Drawing),
            'F' => Ok(TFOperator::Forging),
            'U' => Ok(TFOperator::Upsetting),
            _ => Err(anyhow!("No Such Element").into()),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TFConditionOp {
    Last(TFOperator),       //最后一步为X
    LastSecond(TFOperator), //倒数第二步为X
    LastThird(TFOperator),  //倒数第三步为X
    NotLast(TFOperator),    //非最后步骤为X
    Any(TFOperator),        //任意步骤为X
    None,                   //空
}
impl Into<char> for TFConditionOp {
    fn into(self) -> char {
        match self {
            TFConditionOp::Last(_) => 'L',
            TFConditionOp::LastSecond(_) => 'S',
            TFConditionOp::LastThird(_) => 'T',
            TFConditionOp::NotLast(_) => 'N',
            TFConditionOp::Any(_) => 'A',
            TFConditionOp::None => 'Z', //Zero
        }
    }
}

impl TFConditionOp {
    pub fn is_none(&self) -> bool {
        if let TFConditionOp::None = self {
            true
        } else {
            false
        }
    }
    pub fn make(flag: char, op: TFOperator) -> Result<TFConditionOp, Box<dyn Error>> {
        let c = match flag {
            'L' => Self::Last(op),
            'S' => Self::LastSecond(op),
            'T' => Self::LastThird(op),
            'N' => Self::NotLast(op),
            'A' => Self::Any(op),
            'Z' => Self::None,
            _ => return Err(anyhow!("No Such Char").into()),
        };
        Ok(c)
    }
}
