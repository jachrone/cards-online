use std::io::{self, Write};
use crate::user::Player;

mod user {
    // A public struct with a public field of generic type `T`
    #[derive(Debug)]
    pub struct Player {
        pub name: String,
    }
}


fn main() {
    println!("Hello, welcome to card online");

    print!("Please enter the number of players : ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed before reading input

    let mut player_count_s = String::new();
    io::stdin().read_line(&mut player_count_s).expect("Failed to read line");

    let player_count: i32 = player_count_s.trim().parse().expect("Please type a number!");

    println!("Hello, you will be playing with {} players!", player_count);

    let mut players = Vec::new();
    for i in 1..=player_count {
        print!("Please enter the name of player n°{}: ", i);
        io::stdout().flush().unwrap();

        let mut player_name = String::new();
        io::stdin().read_line(&mut player_name).expect("Failed to read line");

        players.push(
            Player 
            { 
                name: player_name.trim().to_string()
            }
        );

    }
    println!("Players: {:?}", players);
}