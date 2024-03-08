use sensehat::{Colour, SenseHat};

fn main() {
    if let Ok(mut hat) = SenseHat::new() {
        println!("{:?}", hat.get_pressure());
        hat.text("Hi!", Colour::RED, Colour::WHITE);
    }
}
