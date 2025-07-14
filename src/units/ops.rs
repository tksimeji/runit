use std::ops::Div;
use super::types::{Meter, Second, MeterPerSecond};

impl Div<Second> for Meter {
  type Output = MeterPerSecond;

  fn div(self, rhs: Second) -> Self::Output {
    MeterPerSecond(self.0 / rhs.0)
  }
}