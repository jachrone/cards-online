use crate::user::*;
use game::CardColor;
use online_board::*;
use rand::rng;
use rand::seq::SliceRandom;
use std::{fmt, vec};
use std::io::{self, Write};

mod game {
    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum CardType {
        Color,
        Skull,
        Flag,
        Mermaid,
        Pirate,
        SkullKing,
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
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
        pub choices: CardType,
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
        fn card_color(&self) -> Option<CardColor>;
    }

    pub trait Special: Card {}
    pub trait Color: Card {
        fn card_value(&self) -> i32;
    }
    pub trait Atout: Color {}
    pub trait Choice: Card {
        fn card_type_primary(&self) -> CardType;
        fn card_type_choice(&self) -> CardType;
        fn card_type(&self) -> CardType;
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

        fn card_color(&self) -> Option<CardColor> {
            Some(CardColor::Black)
        }
    }

    impl fmt::Display for SkullCard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} Skull\n", self.value)
        }
    }

    impl Color for SkullCard {
        fn card_value(&self) -> i32 {
            self.value
        }
    }

    impl Atout for SkullCard {}

    impl Card for ColorCard {
        fn card_type(&self) -> CardType {
            CardType::Color
        }

        fn card_color(&self) -> Option<CardColor> {
            Some(self.color)
        }
    }

    impl fmt::Display for ColorCard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} {:?}\n", self.value, self.color)
        }
    }

    impl Color for ColorCard {
        fn card_value(&self) -> i32 {
            self.value
        }
    }

    impl Card for WhiteFlagCard {
        fn card_type(&self) -> CardType {
            CardType::Flag
        }

        fn card_color(&self) -> Option<CardColor> {
            Some(CardColor::White)
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

        fn card_color(&self) -> Option<CardColor> {
            Some(CardColor::Brown)
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

        fn card_color(&self) -> Option<CardColor> {
            Some(CardColor::Pink)
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
            CardType::Pirate
        }

        fn card_color(&self) -> Option<CardColor> {
            Some(CardColor::Brown)
        }
    }

    impl fmt::Display for MarySueCard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "MarySue (choices {:?})\n", self.choices)
        }
    }

    impl Choice for MarySueCard {
        fn card_type_primary(&self) -> CardType {
            CardType::Pirate
        }

        fn card_type_choice(&self) -> CardType {
            self.choices
        }

        fn card_type(&self) -> CardType {
            CardType::Pirate
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

        fn card_color(&self) -> Option<CardColor> {
            Some(CardColor::DarkBlue)
        }
    }

    impl fmt::Display for SkullKingCard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "SkullKing\n")
        }
    }

    impl Special for SkullKingCard {}
    impl SkullKing for SkullKingCard {}

    pub fn create_deck(
        card_colors : Vec<CardColor>,
        nb_per_color : i32,
        skull_nb : i32
    ) -> Deck {
        const WHITE_FLAG_NB: i32 = 5;
        const PIRATE_NB: i32 = 5;
        const MERMAID_NB: i32 = 5;

        let mut result = Deck { cards: vec![] };
        
        for color in card_colors {
            for val in 1..= nb_per_color {
                result.cards.push(Box::new(ColorCard {color: color, value : val }));
            }
        }

        for val in 1..=skull_nb {
            result.cards.push(Box::new(SkullCard {value : val }));
        }
        
        for _flag in 0..WHITE_FLAG_NB {
            result.cards.push(Box::new(WhiteFlagCard {}));
        } 

        for _pirate in 0..PIRATE_NB {
            result.cards.push(Box::new(PirateCard {}));
        }
        
        for _mermaid in 0..MERMAID_NB {
            result.cards.push(Box::new(MermaidCard {}));
        }
        
        result.cards.push(Box::new(SkullKingCard {}));
        result.cards.push(Box::new(MarySueCard {choices: CardType::Pirate}));

        return result;
    }

    pub fn create_default_deck() -> Deck {
        Deck {
            cards: vec![
                Box::new(SkullCard { value: 1 }),
                Box::new(SkullCard { value: 2 }),
                Box::new(SkullCard { value: 3 }),
                Box::new(SkullCard { value: 4 }),
                Box::new(SkullCard { value: 5 }),
                Box::new(SkullCard { value: 6 }),
                Box::new(SkullCard { value: 7 }),
                Box::new(SkullCard { value: 8 }),
                Box::new(SkullCard { value: 9 }),
                Box::new(SkullCard { value: 10 }),
                Box::new(SkullCard { value: 11 }),
                Box::new(SkullCard { value: 12 }),
                Box::new(SkullCard { value: 13 }),
                Box::new(ColorCard {
                    color: CardColor::Red,
                    value: 1,
                }),
                Box::new(ColorCard {
                    color: CardColor::Red,
                    value: 2,
                }),
                Box::new(ColorCard {
                    color: CardColor::Red,
                    value: 3,
                }),
                Box::new(ColorCard {
                    color: CardColor::Red,
                    value: 4,
                }),
                Box::new(ColorCard {
                    color: CardColor::Red,
                    value: 5,
                }),
                Box::new(ColorCard {
                    color: CardColor::Red,
                    value: 6,
                }),
                Box::new(ColorCard {
                    color: CardColor::Red,
                    value: 7,
                }),
                Box::new(ColorCard {
                    color: CardColor::Red,
                    value: 8,
                }),
                Box::new(ColorCard {
                    color: CardColor::Red,
                    value: 9,
                }),
                Box::new(ColorCard {
                    color: CardColor::Red,
                    value: 10,
                }),
                Box::new(ColorCard {
                    color: CardColor::Red,
                    value: 11,
                }),
                Box::new(ColorCard {
                    color: CardColor::Red,
                    value: 12,
                }),
                Box::new(ColorCard {
                    color: CardColor::Red,
                    value: 13,
                }),
                Box::new(ColorCard {
                    color: CardColor::Green,
                    value: 1,
                }),
                Box::new(ColorCard {
                    color: CardColor::Green,
                    value: 2,
                }),
                Box::new(ColorCard {
                    color: CardColor::Green,
                    value: 3,
                }),
                Box::new(ColorCard {
                    color: CardColor::Green,
                    value: 4,
                }),
                Box::new(ColorCard {
                    color: CardColor::Green,
                    value: 5,
                }),
                Box::new(ColorCard {
                    color: CardColor::Green,
                    value: 6,
                }),
                Box::new(ColorCard {
                    color: CardColor::Green,
                    value: 7,
                }),
                Box::new(ColorCard {
                    color: CardColor::Green,
                    value: 8,
                }),
                Box::new(ColorCard {
                    color: CardColor::Green,
                    value: 9,
                }),
                Box::new(ColorCard {
                    color: CardColor::Green,
                    value: 10,
                }),
                Box::new(ColorCard {
                    color: CardColor::Green,
                    value: 11,
                }),
                Box::new(ColorCard {
                    color: CardColor::Green,
                    value: 12,
                }),
                Box::new(ColorCard {
                    color: CardColor::Green,
                    value: 13,
                }),
                Box::new(ColorCard {
                    color: CardColor::Blue,
                    value: 1,
                }),
                Box::new(ColorCard {
                    color: CardColor::Blue,
                    value: 2,
                }),
                Box::new(ColorCard {
                    color: CardColor::Blue,
                    value: 3,
                }),
                Box::new(ColorCard {
                    color: CardColor::Blue,
                    value: 4,
                }),
                Box::new(ColorCard {
                    color: CardColor::Blue,
                    value: 5,
                }),
                Box::new(ColorCard {
                    color: CardColor::Blue,
                    value: 6,
                }),
                Box::new(ColorCard {
                    color: CardColor::Blue,
                    value: 7,
                }),
                Box::new(ColorCard {
                    color: CardColor::Blue,
                    value: 8,
                }),
                Box::new(ColorCard {
                    color: CardColor::Blue,
                    value: 9,
                }),
                Box::new(ColorCard {
                    color: CardColor::Blue,
                    value: 10,
                }),
                Box::new(ColorCard {
                    color: CardColor::Blue,
                    value: 11,
                }),
                Box::new(ColorCard {
                    color: CardColor::Blue,
                    value: 12,
                }),
                Box::new(ColorCard {
                    color: CardColor::Blue,
                    value: 13,
                }),
                Box::new(MarySueCard {
                    choices: CardType::Pirate,
                }),
                Box::new(PirateCard {}),
                Box::new(PirateCard {}),
                Box::new(PirateCard {}),
                Box::new(PirateCard {}),
                Box::new(MermaidCard {}),
                Box::new(MermaidCard {}),
                Box::new(SkullKingCard {}),
            ],
        }
    }

    pub fn beats(first : Box<dyn Card>, second : Box<dyn Card>) -> bool
    {
        let first_card_type = first.card_type();
        let second_card_type = second.card_type();

        // Flag in second always loses
        if second_card_type == CardType::Flag {
            return true;
        }

        // for 2 colored cards, the first one defines the required color, and if they match, the highest value wins
        if first_card_type == CardType::Color && second_card_type == CardType::Color {
            if first.card_color() == second.card_color() {
                return first.value > second.value;
            }
            return true;
        }
        else if first_card_type == CardType::Color {
            return false;
        }

        // Skull loses against all Specials and Skulls with higher value
        if first_card_type == CardType::Skull
        {
            if second_card_type == CardType::Skull {
                return first.value > second.value;
            }

            return second_card_type == CardType::Color;
        }

        // Mermaid loses only against a Pirate
        if first_card_type == CardType::Mermaid {
            return second_card_type != CardType::Pirate;
        }

        // Pirate loses only against the SkullKing
        if first_card_type == CardType::Pirate {
            return second_card_type != CardType::SkullKing;
        }

        // SkullKing loses only against a Mermaid
        if first_card_type == CardType::SkullKing {
            return second_card_type != CardType::Mermaid;
        }

        // the first card is a white flag, it loses by default
        return false;
    }
}

mod user {
    use std::fmt;

    // A public struct with a public field of generic type `T`
    #[derive(Debug)]
    pub struct Player {
        pub name: String,
    }

    impl fmt::Display for Player {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "name: {}\n", self.name)
        }
    }
}

mod online_board {
    use std::fmt;

    use crate::{game, user::Player};

    pub struct Seat {
        pub player: Player,
        pub hand: Vec<Box<dyn game::Card>>,
    }

    pub struct Table {
        pub seats: Vec<Seat>,
        pub deck: game::Deck,
        pub seat_count: i32,
    }

    impl fmt::Display for Seat {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.player);
            write!(f, "hand:\n");
            // Print each card in the deck
            Ok(for card in &self.hand {
                write!(f, "{}, ", card);
            })
        }
    }

    impl fmt::Display for Table {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "\tDeck:\n\t\tcards:\n");
            for card in &self.deck.cards {
                write!(f, "\t\t\t{}", card);
            }
            write!(f, "Seats:\n");
            Ok(for seat in &self.seats {
                write!(f, "{}\n", seat);
            })
        }
    }

    pub fn new_table(player_count: i32) -> Table {
        Table {
            seats: Vec::new(),
            deck: game::create_default_deck(),
            seat_count: player_count,
        }
    }

    pub fn new_seat(player: Player) -> Seat {
        Seat {
            player,
            hand: Vec::new(),
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
        }));
    }

    table.deck = game::create_deck(vec![CardColor::Red, CardColor::Blue, CardColor::Green], 13, 13);

    // Shuffle the deck
    let mut rng = rng();
    table.deck.cards.shuffle(&mut rng);

    println!("Table:\n{}", table);
}
