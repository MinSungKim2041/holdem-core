use crate::domain::{chips::Chips, pot::Pot, rule::TableRules, seat::SeatIndex, street::Street};

#[derive(Debug, Clone)]
pub struct PlayerSeatState {
  pub seat: SeatIndex,
  pub stack: Chips,
  pub in_hand: bool,
  pub is_all_in: bool,
  pub acted: bool,
  pub street_contrib: Chips, // 이번 스트리트 총액
  pub hand_contrib: Chips,   // 이번 핸드 총액
}


pub struct HandState {
  pub rules: TableRules,
  pub street: Street,
  pub button: SeatIndex,
  pub acting: SeatIndex,
  pub to_match: Chips, // 콜해야 하는 금액
  pub min_raise_to: Chips, // 최소 레이즈 금액
  pub pot: Pot,
  pub seats: Vec<PlayerSeatState>,
}


impl HandState {
    pub fn seat_mut(&mut self, seat: SeatIndex) -> Option<&mut PlayerSeatState> {
        self.seats.iter_mut().find(|s| s.seat == seat)
    }
    pub fn seat(&self, seat: SeatIndex) -> Option<&PlayerSeatState> {
        self.seats.iter().find(|s| s.seat == seat)
    }
}