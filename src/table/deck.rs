use rand::seq::SliceRandom;
use std::collections::VecDeque;

use super::components::Card;

pub struct Deck {
    cards: VecDeque<Card>,
}

impl Deck {
    pub fn new_shuffled() -> Self {
        let mut cards = Vec::with_capacity(52);
        for i in 0..52 {
            cards.push(Card(i as u8));
        }
        let rng = &mut rand::thread_rng();
        cards.shuffle(rng);
        Self {
            cards: cards.into(),
        }
    }

    pub fn draw(&mut self) -> Card {
        self.cards.pop_front().unwrap()
    }
}
