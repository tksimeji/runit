mod length;
mod time;
mod velocity;

use length::traits::LengthExt;
use length::converters::LengthConversion;
use time:: traits::TimeExt;

fn main() {
    let distance = 100.0.meters().to_millimeters();
    let time = 20.0.seconds();

    let speed = distance / time;

    println!("{}", speed)
}