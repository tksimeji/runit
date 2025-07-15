use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hour<T>(pub T);

impl<T: fmt::Display> fmt::Display for Hour<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} h", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Second<T>(pub T);

impl<T: fmt::Display> fmt::Display for Second<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} s", self.0)
    }
}