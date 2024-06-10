use bevy::prelude::*;

use super::deck::Deck;

#[derive(Resource)]
pub struct DeckResource {
    pub deck: Deck,
}

impl Default for DeckResource {
    fn default() -> Self {
        Self {
            deck: Deck::new_shuffled(),
        }
    }
}
