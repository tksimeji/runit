use crate::length::types::{Centimeter, Kilometer, Meter, Micrometer, Millimeter, Nanometer, Picometer};

pub trait LengthConversion {
    fn to_kilometers(&self) -> Kilometer<f64>;
    fn to_meters(&self) -> Meter<f64>;
    fn to_centimeters(&self) -> Centimeter<f64>;
    fn to_millimeters(&self) -> Millimeter<f64>;
    fn to_micrometers(&self) -> Micrometer<f64>;
    fn to_nanometers(&self) -> Nanometer<f64>;
    fn to_picometers(&self) -> Picometer<f64>;
}

macro_rules! impl_length_conversion {
    (
        $unit:ident, $factor:expr
    ) => {
        impl<T: Into<f64> + Copy> LengthConversion for $unit<T> {
            fn to_kilometers(&self) -> Kilometer<f64> {
                Kilometer(self.0.into() * $factor / 1_000.0)
            }

            fn to_meters(&self) -> Meter<f64> {
                Meter(self.0.into() * $factor)
            }

            fn to_centimeters(&self) -> Centimeter<f64> {
                Centimeter(self.0.into() * $factor * 100.0)
            }

            fn to_millimeters(&self) -> Millimeter<f64> {
                Millimeter(self.0.into() * $factor * 1_000.0)
            }

            fn to_micrometers(&self) -> Micrometer<f64> {
                Micrometer(self.0.into() * $factor * 1_000_000.0)
            }

            fn to_nanometers(&self) -> Nanometer<f64> {
                Nanometer(self.0.into() * $factor * 1_000_000_000.0)
            }

            fn to_picometers(&self) -> Picometer<f64> {
                Picometer(self.0.into() * $factor * 1_000_000_000_000.0)
            }
        }
    };
}


impl_length_conversion!(Kilometer, 1000.0);
impl_length_conversion!(Meter, 1.0);
impl_length_conversion!(Centimeter, 0.01);
impl_length_conversion!(Millimeter, 0.001);
impl_length_conversion!(Micrometer, 0.000_001);
impl_length_conversion!(Nanometer, 0.000_000_001);
impl_length_conversion!(Picometer, 0.000_000_000_001);