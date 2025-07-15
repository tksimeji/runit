use std::fmt;
use std::fmt::Formatter;
use crate::units::unit::Unit;

#[derive(Debug, Clone, PartialEq)]
pub struct Quantity<T> {
    pub value: T,
    pub unit: Unit,
}

impl<T> Quantity<T> {
    pub fn new(value: T, unit: Unit) -> Self {
        Quantity { value, unit }
    }
}

impl <T: fmt::Display> fmt::Display for Quantity<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let num = self.unit.numerators.iter().map(|u| format!("{}", u)).collect::<Vec<_>>().join("*");
        let den = self.unit.denominators.iter().map(|u| format!("{}", u)).collect::<Vec<_>>().join("*");

        if den.is_empty() {
            write!(f, "{} {}", self.value, num)
        } else {
            write!(f, "{} {}/{}", self.value, num, den)
        }
    }
}