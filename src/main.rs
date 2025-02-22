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
use std::sync::Mutex;
use std::{fmt, vec};
use user::*;

static mut GAMEBOARD: Mutex<Table> = Mutex::new(Table {
    seats: Vec::new(),
    river: Vec::new(),
    deck: Deck { cards: Vec::new() },
    seat_count: 0,
});
static mut IS_GAME_STARTED: Mutex<bool> = Mutex::new(false);

fn console_test_run() {
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

#[get("/AddPlayer/<name>")]
fn add_player(name: String) -> String {
    let mut game_board = unsafe { GAMEBOARD.lock().unwrap() };
    let player_id = game_board.seat_count + 1;
    game_board.seats.push(new_seat(Player {
        name: name.clone(),
        player_id: player_id,
    }));
    game_board.seat_count += 1;
    format!("Player {} added to the table", name)
}

#[get("/StartGame")]
fn start_game() -> String {
    let mut is_game_started = unsafe { IS_GAME_STARTED.lock().unwrap() };
    if *is_game_started {
        return "Game already started".to_string();
    }
    *is_game_started = true;

    let mut game_board = unsafe { GAMEBOARD.lock().unwrap() };
    // create a new deck
    game_board.deck = create_default_deck();

    // Shuffle the deck
    let mut rng = rng();
    game_board.deck.cards.shuffle(&mut rng);

    // shuffle the seats
    game_board.seats.shuffle(&mut rng);

    // Collect cards to distribute
    let mut cards = Vec::new();
    for _ in 0..game_board.seats.len() {
        if let Some(card) = game_board.deck.cards.pop() {
            cards.push(card);
        }
    }

    // Distribute one card to each player for the first round
    for (seat, card) in game_board.seats.iter_mut().zip(cards) {
        seat.hand.push(card);
    }

    format!("Game started\n{}", game_board)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, add_player, start_game])
}
