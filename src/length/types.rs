use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Kilometer<T>(pub T);

impl <T: fmt::Display> fmt::Display for Kilometer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} km", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meter<T>(pub T);

impl <T: fmt::Display> fmt::Display for Meter<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} m", self.0)
    }
}