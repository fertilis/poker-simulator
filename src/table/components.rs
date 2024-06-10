use bevy::prelude::*;

/// [[Copy]] because immutable
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SeatIndex(pub u8);

/// [[Copy]] because immutable and used only within other structs
#[derive(Component, Debug, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Chips(pub u32);

/// Nice access to [[Chips]] as the first field of a tuple struct
pub trait Amount
where
    Self: TupleStruct,
{
    fn amount(&self) -> u32 {
        let chips = self
            .field(0)
            .unwrap()
            .as_any()
            .downcast_ref::<Chips>()
            .unwrap();
        chips.0
    }

    fn amount_mut(&mut self) -> &mut u32 {
        let chips = self
            .field_mut(0)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<Chips>()
            .unwrap();
        &mut chips.0
    }

    fn amount_str(&self) -> String {
        let amount = self.amount();
        if amount > 0 {
            format!("{:.1} bb", self.amount() as f32 / 2.0)
        } else {
            "".to_string()
        }
    }
}

#[derive(Component, Reflect, Debug)]
pub struct Stack(pub Chips);

impl Amount for Stack {}

/// Moved chips
#[derive(Component, Reflect, Debug)]
pub struct MovedChips(pub Chips);

impl Amount for MovedChips {}

#[derive(Component, Reflect, Debug)]
pub struct Bankroll(pub Chips);

impl Amount for Bankroll {}

#[derive(Component, Reflect, Debug)]
pub struct Pot(pub Chips);

impl Amount for Pot {}

#[derive(Component, Reflect, Debug)]
pub struct PooledPot(pub Chips);

impl Amount for PooledPot {}

/// 0 to 5 on a 6-max table: number of opponents to act after the player
#[derive(Component, Debug)]
pub struct PreflopPosition(pub u8);

#[derive(Component, Debug)]
pub struct PlayerName(pub String);

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct PlayerIsActive;

#[derive(Component, Debug)]
pub struct PlayerInAction;

#[derive(Component, Debug)]
pub struct PlayerIsHero;

/// 0 to 51
/// Intentionally not [[Copy]]
#[derive(Component, Debug, Clone)]
pub struct Card(pub u8);

impl Default for Card {
    fn default() -> Self {
        Self(255)
    }
}

impl Card {
    pub fn rank(&self) -> u8 {
        self.0 / 4
    }

    pub fn suit(&self) -> u8 {
        if self.0 == 255 {
            4
        } else {
            self.0 % 4
        }
    }

    pub fn rank_str(&self) -> &str {
        if self.0 == 255 {
            "X"
        } else {
            RANKS[self.rank() as usize]
        }
    }

    pub fn suit_color(&self) -> &str {
        if self.0 == 255 {
            return "#ff00ff";
        }
        match self.suit() {
            0 => "#000000",
            1 => "#047804",
            2 => "#0000ff",
            3 => "#ff0000",
            _ => unreachable!(),
        }
    }

    pub fn is_defined(&self) -> bool {
        self.0 < 52
    }
}

const RANKS: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A",
];

#[derive(Component, Debug)]
pub struct HoleCardsFaceUp;

#[derive(Component, Debug)]
pub struct HoleCardsHidden;

#[derive(Component, Debug)]
pub struct HoleCards(pub [Card; 2]);

#[derive(Bundle, Debug)]
pub struct PlayerBundle {
    pub player: Player,
    pub seat_index: SeatIndex,
    pub name: PlayerName,
    pub bankroll: Bankroll,
    pub stack: Stack,
    pub moved_chips: MovedChips,
    pub hole_cards: HoleCards,
    pub preflop_position: PreflopPosition,
    pub is_active: PlayerIsActive,
}

#[derive(Component, Debug)]
pub struct BoardCards(pub [Card; 5]);

#[derive(Component, Debug)]
pub struct Street(pub u8);

#[derive(Bundle, Debug)]
pub struct BoardBundle {
    pub board_cards: BoardCards,
    pub pot: Pot,
    pub pooled_pot: PooledPot,
    pub street: Street,
}
