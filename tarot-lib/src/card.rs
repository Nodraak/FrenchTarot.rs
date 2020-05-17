
// Enum
// canPlay(table) -> bool

use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub enum Card {
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    // TODO
}

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub enum CardSuit {
    S1,
    S2,
    S3,
    S4,
    // TOOD
}

pub type Hand = Vec<Card>;
pub type Dog = Vec<Card>;
pub type Deck = Vec<Card>;

impl Card {
    pub fn deck() -> Deck {
        vec![
            Self::C1,
            Self::C2,
            Self::C3,
            Self::C4,
            Self::C5,
            Self::C6,
            Self::C7,
            Self::C8,
            Self::C9,
            Self::C10,
        ]
    }

    pub fn deal_random(player_count: usize) -> (Vec<Hand>, Dog) {
        let cards_per_players = 10/player_count;

        let mut deck = Card::deck();

        let mut hands = Vec::new();

        for _ in 0..player_count {
            let (selected, remaining) = deck.partial_shuffle(&mut rand::thread_rng(), cards_per_players);
            hands.push(selected.to_vec());
            deck = remaining.to_vec();
        }

        let (selected, remaining) = deck.partial_shuffle(&mut rand::thread_rng(), cards_per_players);

        let dog = selected.to_vec();

        (hands, dog)
    }
}
