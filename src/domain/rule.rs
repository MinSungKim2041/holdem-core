use super::chips::Chips;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitRule { NoLimit }

#[derive(Debug, Clone, Copy)]
pub struct TableRules {
    pub limit: LimitRule,
    pub small_blind: Chips,
    pub big_blind: Chips,
    pub ante: Chips,
}

