use crate::length::types::{Kilometer, Meter, Micrometer, Millimeter, Nanometer};
use crate::time::types::Picosecond;

pub trait LengthExt {
    fn kilometers(self) -> Kilometer<Self> where Self: Sized;
    fn meters(self) -> Meter<Self> where Self: Sized;
    fn millimeters(self) -> Millimeter<Self> where Self: Sized;
    fn micrometers(self) -> Micrometer<Self> where Self: Sized;
    fn nanometers(self) -> Nanometer<Self> where Self: Sized;
    fn picoseconds(self) -> Picosecond<Self> where Self: Sized;
}

impl<T> LengthExt for T where T: Copy {
    fn kilometers(self) -> Kilometer<T> {
        Kilometer(self)
    }

    fn meters(self) -> Meter<T> {
        Meter(self)
    }

    fn millimeters(self) -> Millimeter<T> {
        Millimeter(self)
    }

    fn micrometers(self) -> Micrometer<T> {
        Micrometer(self)
    }

    fn nanometers(self) -> Nanometer<T> {
        Nanometer(self)
    }

    fn picoseconds(self) -> Picosecond<T> {
        Picosecond(self)
    }
}