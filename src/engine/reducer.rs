use crate::{domain::{chips::Chips, seat::SeatIndex, street::Street}, engine::{state::HandState, validator::NormalizedAction}};


pub fn apply(mut state: HandState, seat: SeatIndex, action: NormalizedAction) -> HandState {
  let mut should_advance_turn = true;
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

        let old_to = state.to_match;
        state.to_match = to;
        let raise_size = Chips(to.0 - old_to.0);
        state.min_raise_to = old_to + raise_size;
        state.reset_acted_except(seat);
      }
    }
  }

  state.pot.main += pot_add;
  
  if state.is_betting_round_complete() {
    should_advance_turn = false;
    println!("Betting round complete");
  }

  if should_advance_turn {
    state.advance_turn();
  }

  
  state
}