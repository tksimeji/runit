use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KilometerPerHour<T>(pub T);

impl <T: fmt::Display> fmt::Display for KilometerPerHour<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} km/h", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeterPerSecond<T>(pub T);

impl<T: fmt::Display> fmt::Display for MeterPerSecond<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} m/s", self.0)
    }
}