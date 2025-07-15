use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Kilometer<T>(pub T);

impl <T: fmt::Display> fmt::Display for Kilometer<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} km", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meter<T>(pub T);

impl <T: fmt::Display> fmt::Display for Meter<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} m", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Centimeter<T>(pub T);

impl <T: fmt::Display> fmt::Display for Centimeter<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} cm", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Millimeter<T>(pub T);

impl <T: fmt::Display> fmt::Display for Millimeter<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} mm", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Micrometer<T>(pub T);

impl <T: fmt::Display> fmt::Display for Micrometer<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} μm", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Nanometer<T>(pub T);

impl<T: fmt::Display> fmt::Display for Nanometer<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} nm", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Picometer<T>(pub T);

impl <T: fmt::Display> fmt::Display for Picometer<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} pm", self.0)
    }
}