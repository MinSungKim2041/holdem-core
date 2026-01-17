#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Chips(pub u64);

impl Chips {
  pub const ZERO: Chips = Chips(0);

  pub fn checked_sub(self, rhs: Chips) -> Option<Chips> {
    self.0.checked_sub(rhs.0).map(Chips)
  }

  pub fn saturating_sub(self, rhs: Chips) -> Chips {
    Chips(self.0.saturating_sub(rhs.0))
  }
}


use std::ops::{Add, AddAssign, Sub, SubAssign};
impl Add for Chips {
    type Output = Chips;
    fn add(self, rhs: Chips) -> Chips { Chips(self.0 + rhs.0) }    
}

impl AddAssign for Chips {
  fn add_assign(&mut self, rhs: Chips) {
    self.0 += rhs.0;
  }
}

impl Sub for Chips {
  type Output = Chips;
  fn sub(self, rhs: Chips) -> Chips { Chips(self.0 - rhs.0) }
}

impl SubAssign for Chips {
  fn sub_assign(&mut self, rhs: Chips) {
    self.0 -= rhs.0;
  }
}



