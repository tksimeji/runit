use crate::time::types::{Hour, Second};

pub trait TimeExt {
    fn seconds(self) -> Second<Self> where Self: Sized;
    fn hours(self) -> Hour<Self> where Self: Sized;
}

impl<T> TimeExt for T where T: Copy {
    fn seconds(self) -> Second<Self> {
        Second(self)
    }
    
    fn hours(self) -> Hour<Self> {
        Hour(self)
    }
}