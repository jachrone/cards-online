use std::any::Any;
use std::fmt;
use std::vec;

use rand::seq::SliceRandom;

#[derive(Copy, Clone, PartialEq, Debug)]
enum CardTypeName {
    Color,
    Skull,
    Flag,
    Mermaid,
    Pirate,
    MarySue,
    SkullKing,
}

#[derive(Copy, Clone, Debug)]
pub enum CardType {
    Color(ColorCard),
    Skull(SkullCard),
    Flag(WhiteFlagCard),
    Mermaid(MermaidCard),
    Pirate(PirateCard),
    MarySue(MarySueCard),
    SkullKing(SkullKingCard),
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CardType::Color(card) => write!(f, "{}", card),
            CardType::Skull(card) => write!(f, "{}", card),
            CardType::Flag(card) => write!(f, "{}", card),
            CardType::Mermaid(card) => write!(f, "{}", card),
            CardType::Pirate(card) => write!(f, "{}", card),
            CardType::MarySue(card) => write!(f, "{}", card),
            CardType::SkullKing(card) => write!(f, "{}", card),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CardEffect {
    Pirate,
    Flag,
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
    pub choice: Option<CardEffect>,
}

#[derive(Copy, Clone, Debug)]
pub struct WhiteFlagCard {}

#[derive(Copy, Clone, Debug)]
pub struct SkullKingCard {}

// type PlayCard = Box<dyn Card>;
pub type PlayCard = CardType;

#[derive(Debug, Default, Clone)]
pub struct Deck {
    pub cards: Vec<PlayCard>,
}

impl Deck {
    pub fn new() -> Self {
        create_default_deck()
    }

    pub const fn default() -> Self {
        Deck { cards: Vec::new() }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }
}

// Define the Card trait
pub trait Card: fmt::Display + fmt::Debug + Sync + Send + Clone {
    fn card_type(&self) -> CardTypeName;
    fn card_color(&self) -> CardColor;
    fn card_value(&self) -> Option<i32>;
    fn is_card_special(&self) -> bool;
    fn is_card_atout(&self) -> bool;
    fn set_card_effect(&mut self, _player_choice: CardEffect);
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait Special: Card {
    fn is_card_special(&self) -> bool {
        true
    }
    fn is_card_atout(&self) -> bool {
        false
    }
}
pub trait Color: Card {
    fn is_card_special(&self) -> bool {
        false
    }
    fn is_card_atout(&self) -> bool {
        false
    }
}
pub trait Atout: Color {
    fn is_card_special(&self) -> bool {
        false
    }
    fn is_card_atout(&self) -> bool {
        true
    }
}
pub trait Choice: Card {}
pub trait Pirate: Special {}
pub trait WhiteFlag: Special {}
pub trait MarySue: Choice + Pirate + WhiteFlag {}
pub trait Mermaid: Special {}
pub trait SkullKing: Special {}

// Implement the traits for SkullCards
impl Card for SkullCard {
    fn card_type(&self) -> CardTypeName {
        CardTypeName::Skull
    }

    fn card_color(&self) -> CardColor {
        CardColor::Black
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn card_value(&self) -> Option<i32> {
        Some(self.value)
    }

    fn is_card_special(&self) -> bool {
        false
    }

    fn is_card_atout(&self) -> bool {
        true
    }

    fn set_card_effect(&mut self, _player_choice: CardEffect) {
        panic!("Cannot set card type on SkullCard");
    }
}

impl fmt::Display for SkullCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} Skull\n", self.value)
    }
}

impl Card for ColorCard {
    fn card_type(&self) -> CardTypeName {
        CardTypeName::Color
    }

    fn card_color(&self) -> CardColor {
        self.color
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn card_value(&self) -> Option<i32> {
        Some(self.value)
    }

    fn is_card_special(&self) -> bool {
        false
    }

    fn is_card_atout(&self) -> bool {
        false
    }

    fn set_card_effect(&mut self, _player_choice: CardEffect) {
        panic!("Cannot set card type on ColorCard");
    }
}

impl fmt::Display for ColorCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?}\n", self.value, self.color)
    }
}

impl Card for WhiteFlagCard {
    fn card_type(&self) -> CardTypeName {
        CardTypeName::Flag
    }

    fn card_color(&self) -> CardColor {
        CardColor::White
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn card_value(&self) -> Option<i32> {
        None
    }

    fn is_card_special(&self) -> bool {
        true
    }

    fn is_card_atout(&self) -> bool {
        false
    }

    fn set_card_effect(&mut self, _player_choice: CardEffect) {
        panic!("Cannot set card type on WhiteFlagCard");
    }
}

impl fmt::Display for WhiteFlagCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WhiteFlag\n")
    }
}

impl Card for PirateCard {
    fn card_type(&self) -> CardTypeName {
        CardTypeName::Pirate
    }

    fn card_color(&self) -> CardColor {
        CardColor::Brown
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn card_value(&self) -> Option<i32> {
        None
    }

    fn is_card_special(&self) -> bool {
        true
    }

    fn is_card_atout(&self) -> bool {
        false
    }

    fn set_card_effect(&mut self, _player_choice: CardEffect) {
        panic!("Cannot set card type on PirateCard");
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
    fn card_type(&self) -> CardTypeName {
        CardTypeName::Mermaid
    }

    fn card_color(&self) -> CardColor {
        CardColor::Pink
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn card_value(&self) -> Option<i32> {
        None
    }

    fn is_card_special(&self) -> bool {
        true
    }

    fn is_card_atout(&self) -> bool {
        false
    }

    fn set_card_effect(&mut self, _player_choice: CardEffect) {
        panic!("Cannot set card type on MermaidCard");
    }
}

impl fmt::Display for MermaidCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mermaid\n")
    }
}

impl Card for MarySueCard {
    fn card_type(&self) -> CardTypeName {
        match self.choice {
            Some(CardEffect::Pirate) => CardTypeName::Pirate,
            Some(CardEffect::Flag) => CardTypeName::Flag,
            None => CardTypeName::Pirate,
        }
    }

    fn card_color(&self) -> CardColor {
        CardColor::Brown
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn card_value(&self) -> Option<i32> {
        None
    }

    fn is_card_special(&self) -> bool {
        true
    }

    fn is_card_atout(&self) -> bool {
        false
    }

    fn set_card_effect(&mut self, player_choice: CardEffect) {
        self.choice = Some(player_choice);
    }
}

impl fmt::Display for MarySueCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MarySue (choices {:?})\n", self.choice)
    }
}

impl Card for SkullKingCard {
    fn card_type(&self) -> CardTypeName {
        CardTypeName::SkullKing
    }

    fn card_color(&self) -> CardColor {
        CardColor::DarkBlue
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn card_value(&self) -> Option<i32> {
        None
    }

    fn is_card_special(&self) -> bool {
        true
    }

    fn is_card_atout(&self) -> bool {
        false
    }

    fn set_card_effect(&mut self, _player_choice: CardEffect) {
        panic!("Cannot set card type on SkullKingCard");
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
    card_type: CardTypeName,
    card_number: Option<i32>,
    card_color: Option<CardColor>,
) -> PlayCard {
    match card_type {
        CardTypeName::Color => CardType::Color(ColorCard {
            color: card_color.unwrap(),
            value: card_number.unwrap(),
        }),
        CardTypeName::Skull => CardType::Skull(SkullCard {
            value: card_number.unwrap(),
        }),
        CardTypeName::MarySue => CardType::MarySue(MarySueCard { choice: None }),
        CardTypeName::Flag => CardType::Flag(WhiteFlagCard {}),
        CardTypeName::Mermaid => CardType::Mermaid(MermaidCard {}),
        CardTypeName::Pirate => CardType::Pirate(PirateCard {}),
        CardTypeName::SkullKing => CardType::SkullKing(SkullKingCard {}),
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
                .push(new_card(CardTypeName::Color, Some(val), Some(color)));
        }
    }

    for val in 1..=nb_per_color {
        result
            .cards
            .push(new_card(CardTypeName::Skull, Some(val), None));
    }

    for _flag in 0..WHITE_FLAG_NB {
        result.cards.push(new_card(CardTypeName::Flag, None, None));
    }

    for _pirate in 0..PIRATE_NB {
        result
            .cards
            .push(new_card(CardTypeName::Pirate, None, None));
    }

    for _mermaid in 0..MERMAID_NB {
        result
            .cards
            .push(new_card(CardTypeName::Mermaid, None, None));
    }

    result
        .cards
        .push(new_card(CardTypeName::SkullKing, None, None));
    result
        .cards
        .push(new_card(CardTypeName::MarySue, None, None));

    return result;
}

pub fn create_default_deck() -> Deck {
    create_deck(13)
}

pub fn beats(first: &PlayCard, second: &PlayCard) -> bool {
    match (first, second) {
        (_, CardType::Flag(_)) => return true,
        (CardType::Pirate(_), CardType::SkullKing(_)) => return false,
        (CardType::Pirate(_), _) => return true,
        (CardType::Mermaid(_), CardType::Pirate(_)) => return false,
        (CardType::Mermaid(_), _) => return true, // todo mermaid beats pirate if there is a skullking
        (CardType::SkullKing(_), _) => return true,
        (CardType::Skull(first), CardType::Skull(second)) => {
            return first.card_value().unwrap() > second.card_value().unwrap();
        }
        (CardType::Skull(_), _) => return true,
        (CardType::Color(first), CardType::Color(second)) => {
            if first.card_color() == second.card_color() {
                return first.card_value().unwrap() > second.card_value().unwrap();
            }
            return true;
        }
        (_, _) => return false,
    }
}

#[cfg(test)]
#[test]
fn test_beats() {
    let pirate = new_card(CardTypeName::Pirate, None, None);
    let skull_king = new_card(CardTypeName::SkullKing, None, None);
    let flag = new_card(CardTypeName::Flag, None, None);
    let mermaid = new_card(CardTypeName::Mermaid, None, None);
    let skull = new_card(CardTypeName::Skull, Some(5), None);
    let skull_2 = new_card(CardTypeName::Skull, Some(3), None);
    let color_red_5 = new_card(CardTypeName::Color, Some(5), Some(CardColor::Red));
    let color_red_3 = new_card(CardTypeName::Color, Some(3), Some(CardColor::Red));
    let color_blue_5 = new_card(CardTypeName::Color, Some(5), Some(CardColor::Blue));

    assert!(beats(&pirate, &flag));
    assert!(!beats(&pirate, &skull_king));
    assert!(beats(&pirate, &mermaid));
    assert!(!beats(&mermaid, &pirate));
    assert!(beats(&mermaid, &skull_king));
    assert!(beats(&skull_king, &pirate));
    assert!(beats(&skull, &skull_2));
    assert!(beats(&skull, &color_red_3));
    assert!(beats(&color_red_3, &color_blue_5));
    assert!(beats(&color_red_5, &color_red_3));
    assert!(beats(&color_red_5, &color_blue_5));
    assert!(!beats(&color_red_3, &color_red_5));
}
