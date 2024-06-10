use bevy::prelude::*;

use lazy_static::lazy_static;

use super::{
    base_components::{
        spawn_rounded_rectangle, spawn_rounded_rectangle_with_border, spawn_text, Container,
    },
    card::spawn_card,
    components::{
        BankrollText, CardBack, CardFace, CardRankText, DealerButtonText, DummyLabel,
        HoleCardIndex, InActionIndicator, MovedChipsText, StackText,
    },
    player_attributes::{PlayerAttributes, PlayerAttributesTuple},
};
use crate::table::{
    components::{Amount, HoleCardsFaceUp, Player, SeatIndex},
    events::TableUpdated,
};

lazy_static! {
    static ref PLAYER_POSITIONS: Vec<Vec3> = vec![
        Vec3::new(0.0, 140.0, 4.0),
        Vec3::new(260.0, 70.0, 4.0),
        Vec3::new(260.0, -70.0, 4.0),
        Vec3::new(0.0, -140.0, 4.0),
        Vec3::new(-260.0, -70.0, 4.0),
        Vec3::new(-260.0, 70.0, 4.0),
    ];
    static ref FRONT_POSITIONS: Vec<Vec3> = vec![
        Vec3::new(0.0, -60.0, 0.0),
        Vec3::new(-120.0, -20.0, 0.0),
        Vec3::new(-120.0, 40.0, 0.0),
        Vec3::new(0.0, 80.0, 0.0),
        Vec3::new(120.0, 40.0, 0.0),
        Vec3::new(120.0, -20.0, 0.0),
    ];
    static ref BUTTON_POSITIONS: Vec<Vec3> = vec![
        Vec3::new(70.0, 0.0, 0.0),
        Vec3::new(70.0, 0.0, 0.0),
        Vec3::new(70.0, 0.0, 0.0),
        Vec3::new(-70.0, 0.0, 0.0),
        Vec3::new(-70.0, 0.0, 0.0),
        Vec3::new(-70.0, 0.0, 0.0),
    ];
}

pub fn spawn_players(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    players: &[PlayerAttributesTuple],
) -> Vec<Entity> {
    let mut player_ids = Vec::new();
    for fields in players {
        let player = PlayerAttributes::from(*fields);
        let seat_index = player.seat_index;
        let name = player.player_name;
        let hole_cards = player.hole_cards;
        let root = commands
            .spawn((
                (*seat_index).clone(),
                Container::new(PLAYER_POSITIONS[seat_index.0 as usize].clone()),
            ))
            .id();
        let text_box = spawn_player_text_box(
            commands,
            meshes,
            materials,
            asset_server,
            name.0.as_str(),
            "",
            "",
            *seat_index,
        );
        let in_action_indicator = spawn_rounded_rectangle(
            commands,
            materials,
            meshes,
            Vec3::new(0.0, -28.0, 0.0),
            Vec2::new(90.0, 5.0),
            "#ff00ff",
            2.0,
            (*seat_index, InActionIndicator),
        );
        let front = spawn_text(
            commands,
            asset_server,
            FRONT_POSITIONS[seat_index.0 as usize].clone(),
            "",
            16.0,
            "#ffffff",
            true,
            (*seat_index, MovedChipsText),
        );
        let button = spawn_text(
            commands,
            asset_server,
            BUTTON_POSITIONS[seat_index.0 as usize].clone(),
            "D",
            20.0,
            "#ffff00",
            true,
            (*seat_index, DealerButtonText),
        );
        let face_up = seat_index.0 == 3;
        let hole_card_0 = spawn_card(
            commands,
            meshes,
            materials,
            asset_server,
            &hole_cards.0[0],
            Vec3::new(-15.0, 43.0, 2.0),
            face_up,
            (*seat_index, HoleCardIndex(0)),
        );
        let hole_card_1 = spawn_card(
            commands,
            meshes,
            materials,
            asset_server,
            &hole_cards.0[1],
            Vec3::new(15.0, 43.0, 2.0),
            face_up,
            (*seat_index, HoleCardIndex(1)),
        );

        commands.entity(root).push_children(&[
            text_box,
            in_action_indicator,
            front,
            button,
            hole_card_0,
            hole_card_1,
        ]);
        player_ids.push(root);
    }
    player_ids
}

fn spawn_player_text_box(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    name: &str,
    bankroll: &str,
    stack: &str,
    seat_index: SeatIndex,
) -> Entity {
    let root = commands
        .spawn(Container::new(Vec3::new(0.0, 0.0, 0.0)))
        .id();

    let text_box = spawn_rounded_rectangle_with_border(
        commands,
        materials,
        meshes,
        Vec3::new(0.0, 0.0, 0.0),
        Vec2::new(100.0, 45.0),
        "#777777",
        5.0,
        "#eeeeee",
        2.0,
        DummyLabel,
    );
    let name = spawn_text(
        commands,
        asset_server,
        Vec3::new(0.0, 15.0, 2.0),
        name,
        14.0,
        "#ffffff",
        true,
        seat_index,
    );
    let bankroll = spawn_text(
        commands,
        asset_server,
        Vec3::new(0.0, 1.0, 2.0),
        bankroll,
        14.0,
        "#ffffff",
        true,
        (seat_index, BankrollText),
    );
    let stack = spawn_text(
        commands,
        &asset_server,
        Vec3::new(0.0, -12.0, 2.0),
        stack,
        14.0,
        "#ffffff",
        true,
        (seat_index, StackText),
    );
    commands.entity(root).push_children(&[text_box]);
    commands
        .entity(text_box)
        .push_children(&[name, bankroll, stack]);
    root
}

pub fn update_players(
    mut table_updated_event_reader: EventReader<TableUpdated>,
    mut q_texts: ParamSet<(
        Query<(&mut Text, &SeatIndex), With<BankrollText>>,
        Query<(&mut Text, &SeatIndex), With<StackText>>,
        Query<(&mut Text, &SeatIndex), With<MovedChipsText>>,
        Query<(&mut Text, &SeatIndex), With<DealerButtonText>>,
        Query<(&mut Text, &SeatIndex, &HoleCardIndex), With<CardRankText>>,
    )>,
    mut q_visibilities: ParamSet<(
        Query<(&mut Visibility, &SeatIndex), With<InActionIndicator>>,
        Query<(&mut Visibility, &SeatIndex), With<HoleCardIndex>>,
    )>,
    mut q_card_transforms: ParamSet<(
        Query<(&mut Transform, &SeatIndex), With<CardFace>>,
        Query<(&mut Transform, &SeatIndex), With<CardBack>>,
    )>,
    q_players: Query<PlayerAttributesTuple, With<Player>>,
    q_showdown: Query<(&SeatIndex, Option<&HoleCardsFaceUp>)>,
) {
    let table_updated: bool = table_updated_event_reader.read().last().is_some();
    if !table_updated {
        return;
    }
    let players_ = q_players.iter().collect::<Vec<PlayerAttributesTuple>>();
    let mut players: Vec<PlayerAttributes> = players_
        .iter()
        .map(|x| PlayerAttributes::from(*x))
        .collect();
    players.sort_by_key(|x| x.seat_index.0);

    let mut q_bankroll_text = q_texts.p0();
    for (mut text, seat_index) in q_bankroll_text.iter_mut() {
        let player = &players[seat_index.0 as usize];
        text.sections[0].value = player.bankroll.amount_str();
    }

    let mut q_stack_text = q_texts.p1();
    for (mut text, seat_index) in q_stack_text.iter_mut() {
        let player = &players[seat_index.0 as usize];
        text.sections[0].value = player.stack.amount_str();
    }

    let mut q_moved_chips_text = q_texts.p2();
    for (mut text, seat_index) in q_moved_chips_text.iter_mut() {
        let player = &players[seat_index.0 as usize];
        text.sections[0].value = player.moved_chips.amount_str();
    }

    let mut q_in_action_indicator_visibility = q_visibilities.p0();
    for (mut visibility, seat_index) in q_in_action_indicator_visibility.iter_mut() {
        let player = &players[seat_index.0 as usize];
        if player.in_action.is_some() {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }

    let mut q_hole_card_visibility = q_visibilities.p1();
    for (mut visibility, seat_index) in q_hole_card_visibility.iter_mut() {
        let player = &players[seat_index.0 as usize];
        if player.is_active.is_some() {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }

    let mut q_dealer_button_text = q_texts.p3();
    for (mut text, seat_index) in q_dealer_button_text.iter_mut() {
        let player = &players[seat_index.0 as usize];
        if player.preflop_position.0 == 2 {
            text.sections[0].value = "D".to_string();
        } else {
            text.sections[0].value = "".to_string();
        }
    }

    let mut q_card_rank_text = q_texts.p4();
    for (mut text, seat_index, hole_card_index) in q_card_rank_text.iter_mut() {
        let player = &players[seat_index.0 as usize];
        let card = &player.hole_cards.0[hole_card_index.0];
        text.sections[0].value = card.rank_str().to_string();
        text.sections[0].style.color = Color::hex(card.suit_color()).unwrap();
    }

    for (seat_index, shown) in q_showdown.iter() {
        if seat_index.0 == 3 {
            continue;
        }
        if shown.is_some() {
            for (mut transform, seat_index2) in q_card_transforms.p0().iter_mut() {
                if seat_index.0 == seat_index2.0 {
                    transform.translation.z = 100.0;
                }
            }
            for (mut transform, seat_index2) in q_card_transforms.p1().iter_mut() {
                if seat_index.0 == seat_index2.0 {
                    transform.translation.z = 0.0;
                }
            }
        } else {
            for (mut transform, seat_index2) in q_card_transforms.p0().iter_mut() {
                if seat_index.0 == seat_index2.0 {
                    transform.translation.z = 0.0;
                }
            }
            for (mut transform, seat_index2) in q_card_transforms.p1().iter_mut() {
                if seat_index.0 == seat_index2.0 {
                    transform.translation.z = 100.0;
                }
            }
        }
    }
}
