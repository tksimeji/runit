use std::ops::Div;
use crate::length::types::{Kilometer, Meter, Millimeter};
use crate::time::types::{Hour, Second};
use crate::velocity::types::{KilometerPerHour, MeterPerSecond, MillimeterPerSecond};

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

impl<T: Copy + Div<Output = T>> Div<Second<T>> for Millimeter<T> {
    type Output = MillimeterPerSecond<T>;

    fn div(self, rhs: Second<T>) -> Self::Output {
        MillimeterPerSecond(self.0 / rhs.0)
    }
}