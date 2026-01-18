use crate::domain::{chips::Chips, pot::Pot, rule::TableRules, seat::SeatIndex, street::Street};

/**
 * PlayerSeatState is a struct that represents a player's state in a hand.
 * It contains the player's seat index, stack, in_hand, is_all_in, acted, street_contrib, and hand_contrib.
 * 
 * seat: The seat index of the player.
 * stack: The player's stack of chips.
 * in_hand: Whether the player is in the hand.
 * is_all_in: Whether the player is all in.
 * acted: Whether the player has acted in the current street.
 * street_contrib: The amount of chips the player has contributed to the current street.
 */
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

impl PlayerSeatState {
  pub fn can_act(&self) -> bool {
    self.in_hand && !self.is_all_in
  }
}

/**
 * HandState is a struct that represents the state of a hand.
 * It contains the rules, street, button, acting, to_match, min_raise_to, pot, and seats.
 * 
 * rules: The rules of the hand.
 * street: The street of the hand.
 * button: The seat index of the button.
 * acting: The seat index of the acting player.
 * to_match: The amount of chips the player has to match.
 */
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
    pub fn next_seat_after(&self, from: SeatIndex) -> Option<SeatIndex> {
      if self.seats.is_empty() { return None; }

      let n = self.seats.len();
      let from_pos = self.seats.iter().position(|s| s.seat == from)?;
      for i in 1..=n {
        let idx = (from_pos + i) % n;
        let s = &self.seats[idx];
        if s.can_act() {
          return Some(s.seat);
        }
      }
      None
    }
    pub fn is_betting_round_complete(&self) -> bool {
      let alive = self.seats.iter().filter(|s| s.in_hand).count();
      if alive == 1 { return true; }

      for s in &self.seats {
        if !s.can_act() {
          continue;
        }

        let matched = s.street_contrib.0 == self.to_match.0;
        if !matched || !s.acted {
            return  false;
        }
      }
      
      true
    }
    pub fn reset_acted_except(&mut self, except: SeatIndex) {
      for s in &mut self.seats {
          if s.in_hand && !s.is_all_in && s.seat != except {
              s.acted = false;
          }
      }
  }
  pub fn advance_turn(&mut self) {
    if let Some(next) = self.next_seat_after(self.acting) {
      self.acting = next;
    }
  }
}