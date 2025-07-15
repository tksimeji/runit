use std::ops::Div;
use crate::length::types::Meter;
use crate::time::types::Second;
use crate::velocity::types::MeterPerSecond;

impl<T: Copy + Div<Output = T>> Div<Second<T>> for Meter<T> {
    type Output = MeterPerSecond<T>;

    fn div(self, rhs: Second<T>) -> Self::Output {
        MeterPerSecond(self.0 / rhs.0)
    }
}