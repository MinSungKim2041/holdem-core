use super::chips::Chips;

pub enum PlayerAction {
  Fold,
  Check,
  Raise { amount: Chips },
  Call,
  AllIn,
}

