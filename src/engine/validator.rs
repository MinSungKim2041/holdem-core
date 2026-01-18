use thiserror::Error;

use crate::{domain::{action::PlayerAction, chips::Chips, seat::SeatIndex}, engine::state::HandState};

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RuleViolation {
  #[error("not your turn")]
  NotYourTurn,
  #[error("seat not in hand")]
  NotInHand,
  #[error("check not allowed")]
  CheckNotAllowed,
  #[error("bet/raise too small")]
  TooSmall,
  #[error("insufficient chips")]
  InsufficientChips,
  #[error("invalid state")]
  InvalidState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizedAction {
    Fold,
    Check,
    Call { add: Chips, to: Chips },
    Raise { add: Chips, to: Chips }, 
}

pub fn validate(state: &HandState, seat: SeatIndex, action: PlayerAction) -> Result<NormalizedAction, RuleViolation> {
  if state.acting != seat {
    return Err(RuleViolation::NotYourTurn);
  }
  let ps = state.seat(seat).ok_or(RuleViolation::InvalidState)?;
  if !ps.in_hand { return Err(RuleViolation::NotInHand); }

  let needed_to_call = state.to_match.saturating_sub(ps.street_contrib);

  match action {
    PlayerAction::Fold => Ok(NormalizedAction::Fold),
    PlayerAction::Check => {
      if state.to_match.0 == ps.street_contrib.0 {
        Ok(NormalizedAction::Check)
      } else {
        Err(RuleViolation::CheckNotAllowed)
      }
    }
    PlayerAction::Call => {
      if needed_to_call.0 == 0 {
        Ok(NormalizedAction::Check)
      } else {
        if ps.stack.0 < needed_to_call.0 {
          Ok(NormalizedAction::Call { add: Chips(ps.stack.0), to: Chips(ps.stack.0) + needed_to_call })
        } else {
          Ok(NormalizedAction::Call { add: needed_to_call, to: state.to_match })
        }
      }
    }
    PlayerAction::Raise { amount } => {
      // amount는 "이번 스트리트 총액" 기준으로 받는다고 가정
      if amount.0 <= state.to_match.0 {
          return Err(RuleViolation::TooSmall);
      }
      if amount.0 < state.min_raise_to.0 {
          return Err(RuleViolation::TooSmall);
      }
      let add = Chips(amount.0.saturating_sub(ps.street_contrib.0));
      if add.0 > ps.stack.0 {
          return Err(RuleViolation::InsufficientChips);
      }
      Ok(NormalizedAction::Raise { add, to: amount })
    }
    PlayerAction::AllIn => {
    // all in 에 대한 action 생성해줘
    if ps.stack.0 == 0 {
        return Err(RuleViolation::InsufficientChips);
    }
    
    Ok(NormalizedAction::Raise { add: ps.stack, to: Chips(ps.street_contrib.0) + ps.stack })
    }
  }
}