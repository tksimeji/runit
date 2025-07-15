use crate::units::traits::{LengthExt, TimeExt};

mod units;

fn main() {
    let distance = 100.0.meters();
    let time = 20.0.seconds();
    let speed = distance / time;
    println!("{}", speed)
}