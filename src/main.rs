use rand::Rng;
use reqwest::*;
use std::fmt::format;
use std::io::{self, stdin};
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

/// Define a generic function to get user input
fn collect_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => continue,
        }
    }
}

/// Define a function to get the players
fn collect_players() -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    let mut num_players = 0;
    loop {
        num_players = collect_input::<u32>("How many players (>1) ?");
        if num_players < 2 {
            println!("Invalid X no.! Please try again!");
            continue;
        } else {
            break;
        }
    }
    for i in 1..=num_players {
        let name = collect_input(format!("Player {} name: ", i).as_str());
        players.push(Player { name, score: 0 });
    }
    players
}

/// Define a function to get the max number
fn create_max_range(players: &Vec<Player>) -> u32 {
    players.len() as u32 * 50
}

// Define a function to generate a random number
// M-1: via rand library
// fn generate_number(max_range: u32) -> u32 {
//     rand::thread_rng().gen_range(1..max_range)
// }

#[tokio::main]
// M-2: via API
async fn generate_number(max_range: u32) -> Result<u32> {
    dotenv::from_path("./.env").expect("Failed to load .env file");
    let url = std::env::var("URL")
    .expect("URL var not found")
    .replace("{MAX}", &max_range.to_string());

    let body= reqwest::get(url).await?.text().await?;

    let val = body.trim().parse::<u32>().expect("Error in parsing");
    println!("value = {}", val);

    Ok(val)
}

fn main() {
    let _ = generate_number(100);
    //print!("{}", x);
}
