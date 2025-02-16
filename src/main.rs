use crate::user::*;
use online_board::*;
use rand::rng;
use rand::seq::SliceRandom;
use std::io::{self, Write};
use std::{fmt, vec};

mod game {
    use std::any::Any;

    use super::*;

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum CardType {
        Color,
        Skull,
        Flag,
        Mermaid,
        Pirate,
        MarySue,
        SkullKing,
    }

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum CardColor {
        Red,
        Blue,
        Green,
        Black,
        Brown,
        Pink,
        DarkBlue,
        White,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct ColorCard {
        pub color: CardColor,
        pub value: i32,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct SkullCard {
        pub value: i32,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct MermaidCard {}

    #[derive(Copy, Clone, Debug)]
    pub struct PirateCard {}

    #[derive(Copy, Clone, Debug)]
    pub struct MarySueCard {
        pub choice: Option<CardType>,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct WhiteFlagCard {}

    #[derive(Copy, Clone, Debug)]
    pub struct SkullKingCard {}

    pub struct Deck {
        pub cards: Vec<Box<dyn Card>>,
    }

    // Define the Card trait
    pub trait Card: fmt::Display {
        fn card_type(&self) -> CardType;
        fn card_color(&self) -> CardColor;
        fn card_value(&self) -> Option<i32> {
            None
        }
        fn is_card_special(&self) -> bool {
            false
        }
        fn set_card_type(&mut self, _player_choice: CardType) {}
        fn as_any(&self) -> &dyn std::any::Any;
    }

    pub trait Special: Card {
        fn is_card_special(&self) -> bool {
            true
        }
    }
    pub trait Color: Card {
        fn card_value(&self) -> Option<i32>;
    }
    pub trait Atout: Color {}
    pub trait Choice: Card {
        fn set_card_type(&mut self, player_choice: CardType);
    }
    pub trait Pirate: Special {}
    pub trait WhiteFlag: Special {}
    pub trait MarySue: Choice + Pirate + WhiteFlag {}
    pub trait Mermaid: Special {}
    pub trait SkullKing: Special {}

    // Implement the traits for SkullCards
    impl Card for SkullCard {
        fn card_type(&self) -> CardType {
            CardType::Skull
        }

        fn card_color(&self) -> CardColor {
            CardColor::Black
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl fmt::Display for SkullCard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} Skull\n", self.value)
        }
    }

    impl Color for SkullCard {
        fn card_value(&self) -> Option<i32> {
            Some(self.value)
        }
    }

    impl Atout for SkullCard {}

    impl Card for ColorCard {
        fn card_type(&self) -> CardType {
            CardType::Color
        }

        fn card_color(&self) -> CardColor {
            self.color
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl fmt::Display for ColorCard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} {:?}\n", self.value, self.color)
        }
    }

    impl Color for ColorCard {
        fn card_value(&self) -> Option<i32> {
            Some(self.value)
        }
    }

    impl Card for WhiteFlagCard {
        fn card_type(&self) -> CardType {
            CardType::Flag
        }

        fn card_color(&self) -> CardColor {
            CardColor::White
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl fmt::Display for WhiteFlagCard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "WhiteFlag\n")
        }
    }

    impl Special for WhiteFlagCard {}
    impl WhiteFlag for WhiteFlagCard {}

    impl Card for PirateCard {
        fn card_type(&self) -> CardType {
            CardType::Pirate
        }

        fn card_color(&self) -> CardColor {
            CardColor::Brown
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl fmt::Display for PirateCard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Pirate\n")
        }
    }

    impl Special for PirateCard {}
    impl Pirate for PirateCard {}

    impl Card for MermaidCard {
        fn card_type(&self) -> CardType {
            CardType::Mermaid
        }

        fn card_color(&self) -> CardColor {
            CardColor::Pink
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl fmt::Display for MermaidCard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Mermaid\n")
        }
    }

    impl Special for MermaidCard {}
    impl Mermaid for MermaidCard {}

    impl Card for MarySueCard {
        fn card_type(&self) -> CardType {
            if self.choice.is_some() {
                self.choice.unwrap()
            } else {
                CardType::Pirate
            }
        }

        fn card_color(&self) -> CardColor {
            CardColor::Brown
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl fmt::Display for MarySueCard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "MarySue (choices {:?})\n", self.choice)
        }
    }

    impl Choice for MarySueCard {
        fn set_card_type(&mut self, player_choice: CardType) {
            self.choice = Some(player_choice);
        }
    }

    impl Special for MarySueCard {}
    impl Pirate for MarySueCard {}
    impl WhiteFlag for MarySueCard {}
    impl MarySue for MarySueCard {}

    impl Card for SkullKingCard {
        fn card_type(&self) -> CardType {
            CardType::SkullKing
        }

        fn card_color(&self) -> CardColor {
            CardColor::DarkBlue
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl fmt::Display for SkullKingCard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "SkullKing\n")
        }
    }

    impl Special for SkullKingCard {}
    impl SkullKing for SkullKingCard {}

    fn new_card(
        card_type: CardType,
        card_number: Option<i32>,
        card_color: Option<CardColor>,
    ) -> Box<dyn Card> {
        match card_type {
            CardType::Color => Box::new(ColorCard {
                color: card_color.unwrap(),
                value: card_number.unwrap(),
            }),
            CardType::Skull => Box::new(SkullCard {
                value: card_number.unwrap(),
            }),
            CardType::MarySue => Box::new(MarySueCard { choice: None }),
            CardType::Flag => Box::new(WhiteFlagCard {}),
            CardType::Mermaid => Box::new(MermaidCard {}),
            CardType::Pirate => Box::new(PirateCard {}),
            CardType::SkullKing => Box::new(SkullKingCard {}),
        }
    }

    pub fn create_deck(nb_per_color: i32) -> Deck {
        const WHITE_FLAG_NB: i32 = 5;
        const PIRATE_NB: i32 = 5;
        const MERMAID_NB: i32 = 2;

        let mut result = Deck { cards: vec![] };

        for color in vec![CardColor::Red, CardColor::Blue, CardColor::Green] {
            for val in 1..=nb_per_color {
                result
                    .cards
                    .push(new_card(CardType::Color, Some(val), Some(color)));
            }
        }

        for val in 1..=nb_per_color {
            result
                .cards
                .push(new_card(CardType::Skull, Some(val), None));
        }

        for _flag in 0..WHITE_FLAG_NB {
            result.cards.push(new_card(CardType::Flag, None, None));
        }

        for _pirate in 0..PIRATE_NB {
            result.cards.push(new_card(CardType::Pirate, None, None));
        }

        for _mermaid in 0..MERMAID_NB {
            result.cards.push(new_card(CardType::Mermaid, None, None));
        }

        result.cards.push(new_card(CardType::SkullKing, None, None));
        result.cards.push(new_card(CardType::MarySue, None, None));

        return result;
    }

    pub fn create_default_deck() -> Deck {
        create_deck(13)
    }

    pub fn beats(first: Box<dyn Card>, second: Box<dyn Card>) -> bool {
        let first_card_type = first.card_type();
        let second_card_type = second.card_type();

        match (first_card_type, second_card_type) {
            (_, CardType::Flag) => return true,
            (CardType::Pirate, CardType::SkullKing) => return false,
            (CardType::Pirate, _) => return true,
            (CardType::Mermaid, _) => return true, // todo mermaid beats pirate if there is a skullking
            (CardType::SkullKing, _) => return true,
            (CardType::Skull, CardType::Skull) => {
                return first.card_value().unwrap() > second.card_value().unwrap()
            }
            (CardType::Skull, _) => return true,
            (CardType::Color, CardType::Color) => {
                if first.card_color() == second.card_color() {
                    return first.card_value().unwrap() > second.card_value().unwrap();
                }
                return true;
            }
            (_, _) => return false,
        }
    }
}

mod user {
    use std::fmt;

    // A public struct with a public field of generic type `T`
    #[derive(Debug)]
    pub struct Player {
        pub name: String,
        pub player_id: i32,
    }

    impl fmt::Display for Player {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "name: {}\n", self.name)
        }
    }
}

mod online_board {
    use rand::{rng, seq::SliceRandom};

    use crate::{game, user::Player};
    use std::fmt;

    pub struct Seat {
        pub player: Player,
        pub hand: Vec<Box<dyn game::Card>>,
        pub plis: Vec<Box<dyn game::Card>>,
    }

    pub struct PlayedCard {
        order: i32,
        player_id: i32,
        card: Box<dyn game::Card>,
    }

    pub fn play_card(
        table_river: &mut Vec<PlayedCard>,
        player: &Player,
        card: Box<dyn game::Card>,
    ) {
        let played_card = PlayedCard {
            order: table_river.len() as i32,
            player_id: player.player_id,
            card,
        };
        table_river.push(played_card);
    }

    pub fn store_winner_fold(table: &mut Table, player_id: i32) {
        let mut winner_cards = Vec::new();
        while let Some(card) = table.river.pop() {
            winner_cards.push(card.card);
        }
        if let Some(seat) = table
            .seats
            .iter_mut()
            .find(|seat| seat.player.player_id == player_id)
        {
            seat.plis.extend(winner_cards);
        }
    }

    pub fn clear_table_after_game(table: &mut Table) {
        let mut all_cards = Vec::new();

        // Retrieve cards from seated players
        for seat in table.seats.iter_mut() {
            all_cards.append(&mut seat.hand);
            all_cards.append(&mut seat.plis);
        }

        // Retrieve cards from river if any are left
        while let Some(played_card) = table.river.pop() {
            all_cards.push(played_card.card);
        }

        // Put all cards back in the deck
        table.deck.cards.append(&mut all_cards);

        // Shuffle the deck
        let mut rng = rng();
        table.deck.cards.shuffle(&mut rng);
    }

    pub struct Table {
        pub seats: Vec<Seat>,
        pub deck: game::Deck,
        pub river: Vec<PlayedCard>,
        seat_count: i32,
    }

    impl fmt::Display for Seat {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.player);

            write!(f, "\tplis: {} cards\n", self.plis.len());
            for card in &self.plis {
                write!(f, "\t\t{}", card);
            }

            write!(f, "\thand: {} cards\n", self.hand.len());
            // Print each card in the deck
            Ok(for card in &self.hand {
                write!(f, "\t\t{}", card);
            })
        }
    }

    impl fmt::Display for Table {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "\tDeck:\n\t\t{} cards:\n", self.deck.cards.len());
            for card in &self.deck.cards {
                write!(f, "\t\t\t{}", card);
            }
            write!(f, "Seats:\n");
            for seat in &self.seats {
                write!(f, "{}\n", seat);
            }
            write!(f, "River {} cards:\n", self.river.len());
            Ok(for played_card in &self.river {
                write!(f, "\t{}", played_card.card);
            })
        }
    }

    pub fn new_table(player_count: i32) -> Table {
        Table {
            seats: Vec::new(),
            deck: game::create_default_deck(),
            seat_count: player_count,
            river: Vec::new(),
        }
    }

    pub fn new_seat(player: Player) -> Seat {
        Seat {
            player,
            hand: Vec::new(),
            plis: Vec::new(),
        }
    }
}

fn main() {
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

    table.deck = game::create_default_deck();

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

    online_board::store_winner_fold(&mut table, 1);

    println!("first game Table:\n{}", table);

    for seat in table.seats.iter_mut() {
        if let Some(card) = seat.hand.pop() {
            online_board::play_card(&mut table.river, &seat.player, card);
        }
    }

    println!("mid second game Table:\n{}", table);

    // Clear the table after the game
    online_board::clear_table_after_game(&mut table);

    println!("cleared Table:\n{}", table);
}
