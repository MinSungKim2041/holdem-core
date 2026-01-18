use crate::{domain::{seat::SeatIndex, street::Street}, engine::{state::HandState, validator::NormalizedAction}};


pub fn apply(mut state: HandState, seat: SeatIndex, action: NormalizedAction) -> HandState {

  let mut pot_add = crate::domain::chips::Chips::ZERO;
  {
    let ps = state.seat_mut(seat).expect("seat exists");

    match action {
      NormalizedAction::Fold => {
        ps.in_hand = false;
        ps.acted = true;
      }
      NormalizedAction::Check => {
        ps.acted = true;
      }
      NormalizedAction::Call { add, to: _ } => {
        ps.stack -= add;
        ps.street_contrib += add;
        ps.hand_contrib += add;
        ps.acted = true;
        if ps.stack.0 == 0 { ps.is_all_in = true; }
        pot_add = add;
      }
      NormalizedAction::Raise { add, to } => {
        ps.stack -= add;
        ps.street_contrib = to;
        ps.hand_contrib += add;
        ps.acted = true;
        if ps.stack.0 == 0 { ps.is_all_in = true; }
        pot_add = add;
      }
    }
  }

  state.pot.main += pot_add;
  state
}