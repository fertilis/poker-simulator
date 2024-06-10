use bevy::prelude::*;

use super::{
    components::{
        Bankroll, BoardBundle, BoardCards, Card, Chips, HoleCards, HoleCardsFaceUp, MovedChips,
        Player, PlayerBundle, PlayerInAction, PlayerIsActive, PlayerIsHero, PlayerName, PooledPot,
        Pot, PreflopPosition, SeatIndex, Stack, Street,
    },
    resources::DeckResource,
};

const PLAYER_NAMES: [&str; 6] = ["Adam", "John", "Jane", "You", "Sarah", "Mike"];

const INTITIAL_PREFLOP_POSITIONS: [u8; 6] = [3, 2, 1, 0, 5, 4];

pub fn setup_table(mut commands: Commands, mut deck_resource: ResMut<DeckResource>) {
    let deck = &mut deck_resource.deck;
    for seat_index in 0..6 {
        let blind = if seat_index == 3 {
            Chips(2)
        } else if seat_index == 2 {
            Chips(1)
        } else {
            Chips(0)
        };
        let player_id: Entity = commands
            .spawn(PlayerBundle {
                player: Player {},
                seat_index: SeatIndex(seat_index as u8),
                name: PlayerName(PLAYER_NAMES[seat_index].to_string()),
                bankroll: Bankroll(Chips(2000)),
                stack: Stack(Chips(200)),
                moved_chips: MovedChips(blind),
                hole_cards: HoleCards([deck.draw(), deck.draw()]),
                preflop_position: PreflopPosition(INTITIAL_PREFLOP_POSITIONS[seat_index]),
                is_active: PlayerIsActive {},
            })
            .id();
        if seat_index == 3 {
            commands
                .entity(player_id)
                .insert((PlayerIsHero, HoleCardsFaceUp));
        }
        if seat_index == 4 {
            commands.entity(player_id).insert(PlayerInAction);
            println!("Player in action (setup): {}", PLAYER_NAMES[seat_index]);
        }
    }
    commands.spawn(BoardBundle {
        board_cards: BoardCards([
            Card::default(),
            Card::default(),
            Card::default(),
            Card::default(),
            Card::default(),
        ]),
        pot: Pot(Chips(3)),
        pooled_pot: PooledPot(Chips(0)),
        street: Street(0),
    });
}
