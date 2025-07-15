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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Millisecond<T>(pub T);

impl<T: fmt::Display> fmt::Display for Millisecond<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ms", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Microsecond<T>(pub T);

impl<T: fmt::Display> fmt::Display for Microsecond<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} μs", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Nanosecond<T>(pub T);

impl<T: fmt::Display> fmt::Display for Nanosecond<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ns", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Picosecond<T>(pub T);

impl<T: fmt::Display> fmt::Display for Picosecond<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ps", self.0)
    }
}