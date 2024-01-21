use std::fmt::format;

// Define a struct to represent a player
struct Player {
    name: String,
    score: u32,
}

// Define a trait to represent printable objects
trait Printable {
    fn to_string(&self) -> String;
}

// Implement the Printable trait for the Player struct
impl Printable for Player {
    fn to_string(&self) -> String {
        format!("{} ({})", self.name, self.score)
    }
}

fn main() {
    println!("Hello, world!");
}
