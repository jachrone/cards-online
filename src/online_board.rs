use rand::{rng, seq::SliceRandom};

use crate::{game, user::Player};
use std::fmt;

pub struct Seat {
    pub player: Player,
    pub hand: Vec<Box<dyn game::Card + Sync>>,
    pub plis: Vec<Box<dyn game::Card + Sync>>,
}

pub struct PlayedCard {
    order: i32,
    player_id: i32,
    card: Box<dyn game::Card + Sync>,
}

pub fn play_card(
    table_river: &mut Vec<PlayedCard>,
    player: &Player,
    card: Box<dyn game::Card + Sync>,
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
    pub seat_count: i32,
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
