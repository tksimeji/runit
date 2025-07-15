use crate::time::types::{Hour, Microsecond, Millisecond, Nanosecond, Picosecond, Second};

pub trait TimeExt {
    fn hours(self) -> Hour<Self> where Self: Sized;
    fn seconds(self) -> Second<Self> where Self: Sized;
    fn milliseconds(self) -> Millisecond<Self> where Self: Sized;
    fn microseconds(self) -> Microsecond<Self> where Self: Sized;
    fn nanoseconds(self) -> Nanosecond<Self> where Self: Sized;
    fn picoseconds(self) -> Picosecond<Self> where Self: Sized;
}

impl<T> TimeExt for T where T: Copy {
    fn hours(self) -> Hour<T> {
        Hour(self)
    }

    fn seconds(self) -> Second<T> {
        Second(self)
    }

    fn milliseconds(self) -> Millisecond<T> {
        Millisecond(self)
    }

    fn microseconds(self) -> Microsecond<T> {
        Microsecond(self)
    }

    fn nanoseconds(self) -> Nanosecond<T> {
        Nanosecond(self)
    }

    fn picoseconds(self) -> Picosecond<T> {
        Picosecond(self)
    }
}