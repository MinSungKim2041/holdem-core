
use crate::{domain::{action::PlayerAction, chips::Chips, rule::{LimitRule, TableRules}, seat::SeatIndex, street::Street}, engine::{reducer, state::{HandState, PlayerSeatState}, validator}};

#[test]
fn test_basic() {
  let big_blind = Chips(2);
  let small_blind = Chips(1);
  let rules = TableRules { limit: LimitRule::NoLimit, small_blind, big_blind, ante: Chips(0) };

  let mut state = HandState {
    rules,
    street: Street::Preflop,
    button: SeatIndex(0),
    acting: SeatIndex(1),
    to_match: big_blind,
    min_raise_to: big_blind + big_blind,
    pot: Default::default(),
    seats: vec![
      PlayerSeatState {
        seat: SeatIndex(1),
        stack: Chips(100),
        in_hand: true,
        is_all_in: false,
        acted: false,
        street_contrib: Chips(0),
        hand_contrib: Chips(0),
      }
    ],
  };

  let callAction = validator::validate(&state, SeatIndex(1), PlayerAction::Call).unwrap();
  state = reducer::apply(state, SeatIndex(1), callAction);

  assert_eq!(state.pot.main, Chips(2));
  assert_eq!(state.seat(SeatIndex(1)).unwrap().stack, Chips(98));
  
}