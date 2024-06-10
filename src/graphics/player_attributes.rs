use crate::table::components::{
    Bankroll, HoleCards, HoleCardsHidden, MovedChips, PlayerInAction, PlayerIsActive, PlayerIsHero,
    PlayerName, PreflopPosition, SeatIndex, Stack,
};

pub struct PlayerAttributes<'a> {
    pub seat_index: &'a SeatIndex,
    pub player_name: &'a PlayerName,
    pub bankroll: &'a Bankroll,
    pub stack: &'a Stack,
    pub moved_chips: &'a MovedChips,
    pub hole_cards: &'a HoleCards,
    pub preflop_position: &'a PreflopPosition,
    pub is_active: Option<&'a PlayerIsActive>,
    pub is_hero: Option<&'a PlayerIsHero>,
    pub in_action: Option<&'a PlayerInAction>,
    pub hole_cards_hidden: Option<&'a HoleCardsHidden>,
}

impl From<PlayerAttributesTuple<'_>> for PlayerAttributes<'_> {
    fn from(fields: PlayerAttributesTuple) -> Self {
        unsafe { std::mem::transmute(fields) }
    }
}

pub type PlayerAttributesTuple<'a> = (
    &'a SeatIndex,
    &'a PlayerName,
    &'a Bankroll,
    &'a Stack,
    &'a MovedChips,
    &'a HoleCards,
    &'a PreflopPosition,
    Option<&'a PlayerIsActive>,
    Option<&'a PlayerIsHero>,
    Option<&'a PlayerInAction>,
    Option<&'a HoleCardsHidden>,
);
