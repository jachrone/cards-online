use std::io::{self, Write};

fn main() {
    println!("Hello, welcome to card online");

    print!("Please enter the number of players : ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed before reading input

    let mut player_count_s = String::new();
    io::stdin().read_line(&mut player_count_s).expect("Failed to read line");

    let player_count: i32 = player_count_s.trim().parse().expect("Please type a number!");

    println!("Hello, you will be playing with {} players!", player_count);
}