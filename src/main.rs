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


/// Define a function to get the proximity from the players
/// [proximity = abs_diff(guess, target)]
fn collect_guesses_into_proximities(players: &Vec<Player>, max_range: u32) -> Vec<(String, u32)> {
    let mut player_proximities = Vec::<(String, u32)>::new();
    let target = generate_number(create_max_range(players)).expect("Failure in generating random value");
    // println!("target: {}", target);
    for player in players {
        print!("{}'s turn", player.name);
        let guess = collect_input::<u32>(&format!("Guess the number (1 - {max_range}):"));
        player_proximities.push((player.name.clone(), guess.abs_diff(target) ));
    }
    player_proximities
}


/// Define a function to get the winner
fn get_winner(player_proximities: &Vec<(String, u32)>) -> String {
    player_proximities[0].0.to_owned()
}

/// Define a function to update the scores
fn update_scores(players: &mut Vec<Player>, winner: &str) {
    for player in players {
        if player.name == winner {
            player.score += 1
        }
    }
}

/// Define a function to print the scores
fn print_scores(players: &Vec<Player>) {
    print!("Scores: ");
    for player in players {
        println!("- {}", player.to_string() );
    }
}

/// Define a function to play the game
/// This function starts the Target Proximity Game. It gets the players, creates the max range,
/// and runs the game in a loop until the players decide to stop playing. It prints the winner
/// and updates the scores of the players.
fn play_game(){
    print!("Welcome to the Target Proximity Game! ");
    let mut players = collect_players();
    let max_range = create_max_range(&players);
    
    loop {
        let mut player_proximities = collect_guesses_into_proximities(&players, max_range);
        player_proximities.sort_by_key(|&(_, v)|v);
        let winner = get_winner(&player_proximities);

        println!("Congratulations, {}! You are the winner!", winner);
        update_scores(&mut players, &winner);
        print_scores(&players);

        let play_again: String = collect_input("Play again? (y/n) ");

        // if input is anything other "y", it breaks
        if play_again.to_ascii_lowercase() != "y" {
            break;
        }
    }
}

fn main() {
    play_game();
}
