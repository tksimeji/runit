use std::ops::Div;
use crate::units::quantity::Quantity;

impl<T> Div for Quantity<T> where T: Copy + Div<Output = T> {
  type Output = Quantity<T>;
  
  fn div(self, rhs: Quantity<T>) -> Quantity<T> {
    Quantity {
      value: self.value / rhs.value,
      unit: self.unit.div(&rhs.unit),
    }
  }
}