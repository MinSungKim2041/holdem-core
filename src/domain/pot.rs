use crate::domain::{chips::Chips, seat::SeatIndex};

#[derive(Debug, Clone)]
pub struct SidePot {
  pub amount: Chips,
  pub eligible_players: Vec<SeatIndex>,
}

#[derive(Debug, Clone, Default)]
pub struct Pot {
    pub main: Chips,
    pub sides: Vec<SidePot>,
}

