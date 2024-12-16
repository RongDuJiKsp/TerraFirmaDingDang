pub enum DingDangOperator {
    Tapping,        // 轻击
    Hammering,      // 击打
    HeavyHammering, // 重击
    Drawing,        // 牵拉
    Stamping,       // 冲压
    Bending,        // 弯曲
    Forging,        // 镦锻
    Upsetting,      // 收缩
}

impl DingDangOperator {
    pub fn to_values(&self) -> i32 {
        match self {
            DingDangOperator::Tapping => -3,
            DingDangOperator::Hammering => -6,
            DingDangOperator::Stamping => 2,
            DingDangOperator::Bending => 7,
            DingDangOperator::HeavyHammering => -9,
            DingDangOperator::Drawing => -15,
            DingDangOperator::Forging => 13,
            DingDangOperator::Upsetting => 16,
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            DingDangOperator::Tapping => 'T',
            DingDangOperator::Hammering => 'H',
            DingDangOperator::Stamping => 'S',
            DingDangOperator::Bending => 'B',
            DingDangOperator::HeavyHammering => 'X',
            DingDangOperator::Drawing => 'D',
            DingDangOperator::Forging => 'F',
            DingDangOperator::Upsetting => 'U',
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'T' => Some(Self::Tapping),
            'H' => Some(Self::Hammering),
            'S' => Some(Self::Stamping),
            'B' => Some(Self::Bending),
            'X' => Some(Self::HeavyHammering),
            'D' => Some(Self::Drawing),
            'F' => Some(Self::Forging),
            'U' => Some(Self::Upsetting),
            _ => None,
        }
    }
}
