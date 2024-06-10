use bevy::{prelude::*, time::Stopwatch, utils::hashbrown::HashMap};
use rand::seq::SliceRandom;

use super::{
    betting::{Betting, PlayerAttributes},
    compare_hands::compare_hands,
    components::{
        Amount, Bankroll, BoardCards, Card, HoleCards, HoleCardsFaceUp, HoleCardsHidden,
        MovedChips, Player, PlayerInAction, PlayerIsActive, PlayerIsHero, PooledPot, Pot,
        PreflopPosition, SeatIndex, Stack, Street,
    },
    deck::Deck,
    events::{HeroMoved, TableUpdated},
    resources::DeckResource,
    states::HandoutState,
};

pub fn start_new_handout(
    mut commands: Commands,
    mut next_state: ResMut<NextState<HandoutState>>,
    mut q_players: Query<
        (
            Entity,
            &SeatIndex,
            &mut HoleCards,
            &mut MovedChips,
            &mut Stack,
            &mut Bankroll,
            &mut PreflopPosition,
        ),
        With<Player>,
    >,
    mut q_board: Query<(&mut BoardCards, &mut Pot, &mut PooledPot, &mut Street)>,
    mut deck_resource: ResMut<DeckResource>,
    mut timer: Local<Stopwatch>,
    time: Res<Time>,
) {
    timer.tick(time.delta());
    if timer.elapsed().as_millis() < 1000 {
        return;
    }
    timer.reset();
    deck_resource.deck = Deck::new_shuffled();
    for (
        player_id,
        seat_index,
        mut hole_cards,
        mut moved_chips,
        mut stack,
        mut bankroll,
        mut preflop_position,
    ) in q_players.iter_mut()
    {
        // moved_chips here may contain winnings
        *stack.amount_mut() += moved_chips.amount();
        *moved_chips.amount_mut() = 0;

        if stack.amount() < 2 {
            let top_up_amount = (200 - stack.amount()).min(bankroll.amount());
            *stack.amount_mut() += top_up_amount;
            *bankroll.amount_mut() -= top_up_amount;
        }

        preflop_position.0 = (preflop_position.0 + 1) % 6;
        if preflop_position.0 == 0 {
            // big blind
            *stack.amount_mut() -= 2;
            *moved_chips.amount_mut() += 2;
        }
        if preflop_position.0 == 1 {
            // small blind
            *stack.amount_mut() -= 1;
            *moved_chips.amount_mut() += 1;
        }
        commands.entity(player_id).insert(PlayerIsActive);
        if preflop_position.0 == 5 {
            commands.entity(player_id).insert(PlayerInAction);
        }
        if seat_index.0 != 3 {
            commands.entity(player_id).remove::<HoleCardsFaceUp>();
        }
        hole_cards.0[0] = deck_resource.deck.draw();
        hole_cards.0[1] = deck_resource.deck.draw();
    }
    let (mut board_cards, mut pot, mut pooled_pot, mut street) = q_board.single_mut();
    for i in 0..5 {
        board_cards.0[i] = Card::default();
    }
    *pot.amount_mut() = 3;
    *pooled_pot.amount_mut() = 0;
    street.0 = 0;
    next_state.set(HandoutState::ExpectingMove);
}

pub fn make_move(
    mut commands: Commands,
    mut next_state: ResMut<NextState<HandoutState>>,
    mut players_queries: ParamSet<(
        Query<(Entity, &mut Stack, &mut MovedChips), With<PlayerInAction>>,
        Query<PlayerAttributes, With<Player>>,
    )>,
    mut q_pot: Query<&mut Pot>,
    mut table_updated_event_writer: EventWriter<TableUpdated>,
    mut hero_moved_event_reader: EventReader<HeroMoved>,
    mut timer: Local<Stopwatch>,
    q_street: Query<&Street>,
    q_hero: Query<Option<&PlayerIsHero>, With<PlayerInAction>>,
    time: Res<Time>,
) {
    let player_is_hero = q_hero.single().is_some();

    let move_amount = if player_is_hero {
        if let Some(hero_moved_event) = hero_moved_event_reader.read().next() {
            (hero_moved_event.0).0
        } else {
            return;
        }
    } else {
        timer.tick(time.delta());
        if timer.elapsed().as_millis() < 1000 {
            return;
        }
        timer.reset();
        let street = q_street.single().0;
        let q_players = players_queries.p1();
        let players = q_players.iter().collect::<Vec<_>>();
        let betting = Betting::new(&players, q_pot.single().amount(), street);
        let options: Vec<i32> = betting.move_options();
        let mut rng = rand::thread_rng();
        let weights = if street == 0 {
            vec![0.7, 0.2, 0.1]
        } else {
            if options.len() == 2 {
                vec![0.5, 0.5]
            } else {
                vec![0.6, 0.3, 0.1]
            }
        };
        let weights_map = options
            .iter()
            .zip(weights.iter())
            .map(|(i, w)| (*i, *w))
            .collect::<HashMap<i32, f32>>();
        let move_amount: i32 = *options
            .choose_weighted(&mut rng, |i| weights_map[i])
            .unwrap();
        move_amount
    };

    let q_actor = players_queries.p0();
    let (actor_id, _, _) = q_actor.single();
    if move_amount < 0 {
        commands.entity(actor_id).remove::<PlayerIsActive>();
        commands.entity(actor_id).insert(HoleCardsHidden);
    } else {
        let mut pot = q_pot.single_mut();
        let mut q_actor = players_queries.p0();
        let (_, mut stack, mut moved_chips) = q_actor.single_mut();
        let positive_move_amount = move_amount as u32;
        *moved_chips.amount_mut() += positive_move_amount;
        if positive_move_amount <= stack.amount() {
            *stack.amount_mut() -= positive_move_amount;
        } else {
            *stack.amount_mut() = 0;
        }
        *pot.amount_mut() += positive_move_amount;
    }
    commands.entity(actor_id).remove::<PlayerInAction>();

    let q_players = players_queries.p1();
    let players = q_players.iter().collect::<Vec<_>>();

    // Chip amounts are changed, but the fact that actor folded is not yet reflected in q_players
    let mut betting = Betting::new(&players, q_pot.single().amount(), q_street.single().0);
    if move_amount < 0 {
        betting.set_actor_inactive();
    }
    let next_actor_id = betting.next_actor_id();

    match next_actor_id {
        Some(next_actor_id) => {
            commands.entity(next_actor_id).insert(PlayerInAction);
            next_state.set(HandoutState::ExpectingMove);
        }
        None => {
            next_state.set(HandoutState::ExpectingPool);
        }
    }
    table_updated_event_writer.send(TableUpdated);
}

pub fn pool_moved_chips(
    mut next_state: ResMut<NextState<HandoutState>>,
    mut q_players: Query<(Entity, &mut MovedChips, Option<&PlayerIsActive>), With<Player>>,
    mut q_pot: Query<&mut PooledPot>,
    mut table_updated_event_writer: EventWriter<TableUpdated>,
    q_street: Query<&Street>,
) {
    let street: u8 = q_street.single().0;
    let mut n_active = 0;
    for (_player_id, mut moved_chips, is_active) in q_players.iter_mut() {
        let mut pooled_pot = q_pot.single_mut();
        *pooled_pot.amount_mut() += moved_chips.amount();
        *moved_chips.amount_mut() = 0;
        table_updated_event_writer.send(TableUpdated);
        if is_active.is_some() {
            n_active += 1;
        }
    }
    if n_active == 1 {
        next_state.set(HandoutState::ExpectingWinningsAttribution);
    } else if street == 3 {
        next_state.set(HandoutState::ExpectingShowdown);
    } else {
        next_state.set(HandoutState::ExpectingDeal);
    }
}

pub fn deal_community_cards(
    mut commands: Commands,
    mut next_state: ResMut<NextState<HandoutState>>,
    mut deck_resource: ResMut<DeckResource>,
    mut table_updated_event_writer: EventWriter<TableUpdated>,
    mut q_board: Query<&mut BoardCards>,
    mut q_street: Query<&mut Street>,
    q_players: Query<PlayerAttributes, With<Player>>,
    q_pot: Query<&Pot>,
) {
    let mut board_cards: Mut<BoardCards> = q_board.single_mut();
    let cards: &mut [Card; 5] = &mut board_cards.0;
    let mut street: Mut<Street> = q_street.single_mut();
    if !cards[0].is_defined() {
        // Preflop
        cards[0] = deck_resource.deck.draw();
        cards[1] = deck_resource.deck.draw();
        cards[2] = deck_resource.deck.draw();
        street.0 = 1;
    } else if !cards[3].is_defined() {
        // Flop
        cards[3] = deck_resource.deck.draw();
        street.0 = 2;
    } else if !cards[4].is_defined() {
        // Turn
        cards[4] = deck_resource.deck.draw();
        street.0 = 3;
    } else {
        // River
        unreachable!();
    }
    table_updated_event_writer.send(TableUpdated);

    let pot = q_pot.single();
    let players = q_players.iter().collect::<Vec<_>>();
    let betting = Betting::new(&players, pot.amount(), q_street.single().0);

    let next_actor_id = betting.next_actor_id().unwrap(); // guaranteed to exist
    commands.entity(next_actor_id).insert(PlayerInAction);
    next_state.set(HandoutState::ExpectingMove);
}

pub fn do_showdown(
    mut commands: Commands,
    mut next_state: ResMut<NextState<HandoutState>>,
    q_opponents: Query<Entity, (With<PlayerIsActive>, Without<PlayerIsHero>)>,
) {
    for opponent_id in q_opponents.iter() {
        commands.entity(opponent_id).insert(HoleCardsFaceUp);
    }
    next_state.set(HandoutState::ShowdownMade);
}

pub fn attribute_winnings(
    mut next_state: ResMut<NextState<HandoutState>>,
    mut table_updated_event_writer: EventWriter<TableUpdated>,
    mut q_players: Query<(Entity, &HoleCards, &mut MovedChips), With<PlayerIsActive>>,
    mut q_pot: Query<&mut Pot>,
    mut timer: Local<Stopwatch>,
    q_board: Query<&BoardCards>,
    time: Res<Time>,
) {
    timer.tick(time.delta());
    if timer.elapsed().as_millis() < 500 {
        return;
    }
    timer.reset();
    let players = q_players.iter().collect::<Vec<_>>();
    if players.len() == 1 {
        for (_player_id, _, mut moved_chips) in q_players.iter_mut() {
            *moved_chips.amount_mut() += q_pot.single().amount();
            *q_pot.single_mut().amount_mut() = 0;
            table_updated_event_writer.send(TableUpdated);
        }
    } else {
        let board_cards: &BoardCards = q_board.single();
        let mut full_hands: Vec<Vec<Card>> = (0..players.len())
            .map(|_| board_cards.0.to_vec().clone())
            .collect();
        for (i, (_, hole_cards, _)) in players.iter().enumerate() {
            full_hands[i].extend(hole_cards.0.iter().map(|x| (*x).clone()));
        }
        let winning_hand_indices = compare_hands(&full_hands);
        let win_amount: u32 = if winning_hand_indices.len() > 0 {
            q_pot.single().amount() / winning_hand_indices.len() as u32
        } else {
            0
        };
        for winning_hand_index in winning_hand_indices {
            for (i, (_player_id, _, mut moved_chips)) in q_players.iter_mut().enumerate() {
                if i == winning_hand_index {
                    *moved_chips.amount_mut() += win_amount;
                }
            }
        }
        *q_pot.single_mut().amount_mut() = 0;
        table_updated_event_writer.send(TableUpdated);
    }
    next_state.set(HandoutState::HandoutEnded);
}

pub fn on_showdown_made(
    mut next_state: ResMut<NextState<HandoutState>>,
    mut timer: Local<Stopwatch>,
    time: Res<Time>,
) {
    timer.tick(time.delta());
    if timer.elapsed().as_millis() < 1000 {
        return;
    }
    timer.reset();
    next_state.set(HandoutState::ExpectingWinningsAttribution);
}
