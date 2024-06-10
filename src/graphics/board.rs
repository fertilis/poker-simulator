use bevy::prelude::*;

use crate::table::{
    components::{Amount, BoardCards, Card, PooledPot, Pot},
    events::TableUpdated,
};

use super::{
    base_components::{spawn_text, Container},
    card::spawn_card,
    components::{BoardCardIndex, CardRankText, PotText},
};

pub type BoardAttributes<'a> = (&'a Pot, &'a PooledPot, &'a BoardCards);

pub fn spawn_board(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    board: &[BoardAttributes],
) -> Entity {
    let root = commands
        .spawn(Container::new(Vec3::new(0.0, 10.0, 0.0)))
        .id();

    let (_, _, board_cards) = board[0];

    let pot_text = spawn_text(
        commands,
        asset_server,
        Vec3::new(0.0, -30.0, 3.0),
        "",
        16.0,
        "#ffffff",
        true,
        PotText,
    );
    let face_up = true;
    let mut children: Vec<Entity> = board_cards
        .0
        .iter()
        .enumerate()
        .map(|(index, card)| {
            spawn_card(
                commands,
                meshes,
                materials,
                asset_server,
                card,
                Vec3::new(-56.0 + index as f32 * 28.0, 10.0, 2.0),
                face_up,
                BoardCardIndex(index),
            )
        })
        .collect();
    children.push(pot_text);
    commands.entity(root).push_children(&children);

    root
}

pub fn update_board(
    mut table_updated_event_reader: EventReader<TableUpdated>,
    mut queries: ParamSet<(
        Query<(&mut Visibility, &BoardCardIndex)>,
        Query<(&mut Text, &BoardCardIndex), With<CardRankText>>,
        Query<&mut Text, With<PotText>>,
    )>,
    q_board: Query<BoardAttributes>,
) {
    let table_updated: bool = table_updated_event_reader.read().last().is_some();
    if !table_updated {
        return;
    }
    let (pot, _, board_cards) = q_board.single();
    let mut q_pot_text = queries.p2();
    let mut pot_text = q_pot_text.single_mut();
    pot_text.sections[0].value = format!("Pot: {}", pot.amount_str());

    let mut q_card_visibility = queries.p0();
    for (mut visibility, card_index) in q_card_visibility.iter_mut() {
        let card: &Card = &board_cards.0[card_index.0];
        if card.is_defined() {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }

    let mut q_card_rank_text = queries.p1();
    for (mut text, card_index) in q_card_rank_text.iter_mut() {
        let card: &Card = &board_cards.0[card_index.0];
        text.sections[0].value = card.rank_str().to_string();
        text.sections[0].style.color = Color::hex(card.suit_color()).unwrap();
    }
}
