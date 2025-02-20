#[macro_use]
extern crate rocket;

mod game;
mod online_board;
mod user;

use game::*;
use online_board::*;
use rand::rng;
use rand::seq::SliceRandom;
use std::io::{self, Write};
use std::{fmt, vec};
use user::*;

fn console_run() {
    println!("Hello, welcome to card online");

    print!("Please enter the number of players: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed before reading input

    let mut player_count_s = String::new();
    io::stdin()
        .read_line(&mut player_count_s)
        .expect("Failed to read line");

    let player_count: i32 = player_count_s
        .trim()
        .parse()
        .expect("Please type a number!");

    println!("Hello, you will be playing with {} players!", player_count);

    let mut table = new_table(player_count);

    for i in 1..=player_count {
        print!("Please enter the name of player n°{}: ", i);
        io::stdout().flush().unwrap();

        let mut player_name = String::new();
        io::stdin()
            .read_line(&mut player_name)
            .expect("Failed to read line");

        table.seats.push(new_seat(Player {
            name: player_name.trim().to_string(),
            player_id: i,
        }));
    }

    table.deck = create_default_deck();

    println!("Fresh Table:\n{}", table);

    // Shuffle the deck
    let mut rng = rng();
    table.deck.cards.shuffle(&mut rng);

    println!("Shuffled deck Table:\n{}", table);

    //teeeeests
    // Distribute two card to each players
    for seat in table.seats.iter_mut() {
        if let Some(card) = table.deck.cards.pop() {
            seat.hand.push(card);
        }
        // and again ... and again (spoon joke)
        if let Some(card) = table.deck.cards.pop() {
            seat.hand.push(card);
        }
    }

    for seat in table.seats.iter_mut() {
        if let Some(card) = seat.hand.pop() {
            online_board::play_card(&mut table.river, &seat.player, card);
        }
    }

    store_winner_fold(&mut table, 1);

    println!("first game Table:\n{}", table);

    for seat in table.seats.iter_mut() {
        if let Some(card) = seat.hand.pop() {
            online_board::play_card(&mut table.river, &seat.player, card);
        }
    }

    println!("mid second game Table:\n{}", table);

    // Clear the table after the game
    clear_table_after_game(&mut table);

    println!("cleared Table:\n{}", table);
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    console_run();
    rocket::build().mount("/", routes![index])
}
