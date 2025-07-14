use super::types::{Meter, Second};

pub trait LengthExt {
  fn meters(self) -> Meter;
}

impl LengthExt for f64 {
  fn meters(self) -> Meter {
    Meter(self)
  }
}

pub trait TimeExt {
  fn seconds(self) -> Second;
}

impl TimeExt for f64 {
  fn seconds(self) -> Second {
    Second(self)
  }
}