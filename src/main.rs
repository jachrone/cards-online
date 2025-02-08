use crate::user::Player;
use std::fmt;
use std::io::{self, Write};

mod game {
    use std::fmt;

    #[derive(Copy, Clone, Debug)]
    pub enum CardType {
        Color,
        Skull,
        Flag,
        Mermaid,
        Pirate,
        SkullKing,
    }

    #[derive(Copy, Clone, Debug)]
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
    pub struct MarySueCard {
        pub choice: CardType,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct WhiteFlagCard {}

    #[derive(Copy, Clone, Debug)]
    pub struct SkullKingCard {}

    pub struct Deck {
        pub cards: Vec<Box<dyn Card>>,
    }

    // Define the Special trait
    pub trait Card: fmt::Display {
        fn card_type(&self) -> CardType;
        fn card_color(&self) -> CardColor;
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
        fn card_color(&self) -> CardColor {
            CardColor::Black
        }
    }
    impl Color for SkullCard {
        fn card_value(&self) -> i32 {
            self.value
        }
    }

    impl Atout for SkullCard {}

    impl fmt::Display for SkullCard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "SkullCard with value: {}", self.value)
        }
    }

    impl Card for ColorCard {
        fn card_type(&self) -> CardType {
            CardType::Color
        }
        fn card_color(&self) -> CardColor {
            self.color
        }
    }
    impl Color for ColorCard {
        fn card_value(&self) -> i32 {
            self.value
        }
    }
    impl fmt::Display for ColorCard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "ColorCard with color: {:?} and value: {}",
                self.color, self.value
            )
        }
    }
    impl Card for WhiteFlagCard {
        fn card_type(&self) -> CardType {
            CardType::Flag
        }
        fn card_color(&self) -> CardColor {
            CardColor::White
        }
    }
    impl Special for WhiteFlagCard {}
    impl WhiteFlag for WhiteFlagCard {}
    impl fmt::Display for WhiteFlagCard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "WhiteFlagCard")
        }
    }

    impl Card for PirateCard {
        fn card_type(&self) -> CardType {
            CardType::Pirate
        }
        fn card_color(&self) -> CardColor {
            CardColor::Brown
        }
    }
    impl Special for PirateCard {}
    impl Pirate for PirateCard {}
    impl fmt::Display for PirateCard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "PirateCard")
        }
    }

    impl Card for MermaidCard {
        fn card_type(&self) -> CardType {
            CardType::Mermaid
        }
        fn card_color(&self) -> CardColor {
            CardColor::Pink
        }
    }
    impl Special for MermaidCard {}
    impl Mermaid for MermaidCard {}
    impl fmt::Display for MermaidCard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "MermaidCard")
        }
    }
    impl Card for MarySueCard {
        fn card_type(&self) -> CardType {
            CardType::Pirate
        }
        fn card_color(&self) -> CardColor {
            CardColor::Brown
        }
    }
    impl Choice for MarySueCard {
        fn card_type_primary(&self) -> CardType {
            CardType::Pirate
        }
        fn card_type_choice(&self) -> CardType {
            self.choice
        }
        fn card_type(&self) -> CardType {
            CardType::Pirate
        }
    }
    impl Special for MarySueCard {}
    impl Pirate for MarySueCard {}
    impl WhiteFlag for MarySueCard {}
    impl MarySue for MarySueCard {}
    impl fmt::Display for MarySueCard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "MarySueCard with choice: {:?}", self.choice)
        }
    }

    impl Card for SkullKingCard {
        fn card_type(&self) -> CardType {
            CardType::SkullKing
        }
        fn card_color(&self) -> CardColor {
            CardColor::DarkBlue
        }
    }
    impl Special for SkullKingCard {}
    impl SkullKing for SkullKingCard {}
    impl fmt::Display for SkullKingCard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "SkullKingCard")
        }
    }

    pub fn create_deck() -> Deck {
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
                    choice: CardType::Pirate,
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
}

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
    io::stdin()
        .read_line(&mut player_count_s)
        .expect("Failed to read line");

    let player_count: i32 = player_count_s
        .trim()
        .parse()
        .expect("Please type a number!");

    println!("Hello, you will be playing with {} players!", player_count);

    let mut players = Vec::new();
    for i in 1..=player_count {
        print!("Please enter the name of player n°{}: ", i);
        io::stdout().flush().unwrap();

        let mut player_name = String::new();
        io::stdin()
            .read_line(&mut player_name)
            .expect("Failed to read line");

        players.push(Player {
            name: player_name.trim().to_string(),
        });
    }
    println!("Players: {:?}", players);

    let deck = game::create_deck();

    for card in &deck.cards {
        println!("{}", card);
    }
}
