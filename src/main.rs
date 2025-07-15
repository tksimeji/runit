mod length;
mod time;
mod velocity;

use length::traits::LengthExt;
use time:: traits::TimeExt;

fn main() {
    let distance = 100.0.meters();
    let time = 20.0.seconds();

    let speed = distance / time;

    println!("{}", speed)
}