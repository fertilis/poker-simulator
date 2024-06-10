use bevy::prelude::*;

use crate::table::components::Card;

use super::{
    base_components::{
        spawn_rounded_rectangle, spawn_rounded_rectangle_with_border, spawn_text, Container,
    },
    components::{CardBack, CardFace, CardRankText},
};

pub fn spawn_card<T: Bundle + Clone>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    card: &Card,
    pos: Vec3,
    face_up: bool,
    label: T,
) -> Entity {
    let root = commands.spawn((label.clone(), Container::new(pos))).id();
    let card_face_box = spawn_rounded_rectangle(
        commands,
        materials,
        meshes,
        Vec3::new(0.0, 0.0, if face_up { 3.0 } else { 0.0 }),
        Vec2::new(25.0, 35.0),
        "#eeeeee",
        4.0,
        (label.clone(), CardFace),
    );
    let card_rank_text = spawn_text(
        commands,
        asset_server,
        Vec3::new(0.0, 2.0, 1.0),
        &card.rank_str(),
        30.0,
        card.suit_color(),
        true,
        (label.clone(), CardRankText),
    );
    commands
        .entity(card_face_box)
        .push_children(&[card_rank_text]);

    let card_back_box = spawn_rounded_rectangle_with_border(
        commands,
        materials,
        meshes,
        Vec3::new(0.0, 0.0, if face_up { 0.0 } else { 3.0 }),
        Vec2::new(25.0, 35.0),
        "#a61600",
        4.0,
        "#eeeeee",
        2.0,
        (label.clone(), CardBack),
    );

    commands
        .entity(root)
        .push_children(&[card_face_box, card_back_box]);
    root
}
