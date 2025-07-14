use crate::units::traits::{LengthExt, TimeExt};

mod units;

fn main() {
    let speed = 100.0.meters() / 20.0.seconds();
    println!("{:?}", speed);
}