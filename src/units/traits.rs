use crate::units::quantity::Quantity;
use crate::units::unit::{Unit, UnitAtom};

pub trait LengthExt {
  fn meters(self) -> Quantity<Self> where Self: Sized;
}

impl<T> LengthExt for T where T: Copy {
  fn meters(self) -> Quantity<T> {
    Quantity::new(self, Unit {
      numerators: vec![UnitAtom::Meter],
      denominators: vec![],
    })
  }
}

pub trait TimeExt {
  fn seconds(self) -> Quantity<Self> where Self: Sized;
}

impl<T> TimeExt for T where T: Copy {
  fn seconds(self) -> Quantity<T> {
    Quantity::new(self, Unit {
      numerators: vec![UnitAtom::Second],
      denominators: vec![],
    })
  }
}