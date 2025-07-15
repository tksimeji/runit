use std::ops::Div;
use crate::length::types::{Kilometer, Meter};
use crate::time::types::{Hour, Second};
use crate::velocity::types::{KilometerPerHour, MeterPerSecond};

impl<T: Copy + Div<Output = T>> Div<Hour<T>> for Kilometer<T> {
    type Output = KilometerPerHour<T>;

    fn div(self, rhs: Hour<T>) -> Self::Output {
        KilometerPerHour(self.0 / rhs.0)
    }
}

impl<T: Copy + Div<Output = T>> Div<Second<T>> for Meter<T> {
    type Output = MeterPerSecond<T>;

    fn div(self, rhs: Second<T>) -> Self::Output {
        MeterPerSecond(self.0 / rhs.0)
    }
}