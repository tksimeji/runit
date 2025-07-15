use crate::length::types::{Kilometer, Meter};

pub trait LengthExt {
    fn meters(self) -> Meter<Self> where Self: Sized;
    fn kilometers(self) -> Kilometer<Self> where Self: Sized;
}

impl<T> LengthExt for T where T: Copy {
    fn meters(self) -> Meter<T> {
        Meter(self)
    }
    
    fn kilometers(self) -> Kilometer<T> {
        Kilometer(self)
    }
}