use bevy::prelude::*;

use crate::table::{components::Player, events::TableUpdated};

use super::{
    base_components::{spawn_text, Container, MaterialMesh},
    board::{spawn_board, BoardAttributes},
    components::{InfoMessage, TableRoot, TableTop, WindowBackground},
    move_controls::spawn_move_controls,
    pause::spawn_pause_button,
    player::spawn_players,
    player_attributes::PlayerAttributesTuple,
};

const TABLE_TOP_Y: f32 = 70.0;

pub fn setup_table_ui(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut table_updated_event_writer: EventWriter<TableUpdated>,
    asset_server: Res<AssetServer>,
    q_players: Query<PlayerAttributesTuple, With<Player>>,
    q_board: Query<BoardAttributes>,
) {
    let background = spawn_background(&mut commands, &mut meshes, &mut color_materials);
    let table_top = spawn_table_top(&mut commands, &mut meshes, &mut color_materials);

    let table_root = commands
        .spawn((TableRoot, Container::new(Vec3::new(0.0, 0.0, 0.0)).hidden()))
        .id();

    commands.entity(table_top).push_children(&[table_root]);

    //    let info_message = spawn_info_message(&mut commands, &asset_server);
    let pause_button = spawn_pause_button(
        &mut commands,
        &mut meshes,
        &mut color_materials,
        &asset_server,
        Vec3::new(0.0, -250.0, 3.0),
    );
    commands
        .entity(background)
        .push_children(&[table_top, pause_button]);

    let players: Vec<PlayerAttributesTuple> = q_players.iter().collect();
    let mut children: Vec<Entity> = spawn_players(
        &mut commands,
        &mut meshes,
        &mut color_materials,
        &asset_server,
        &players,
    );
    let board: Vec<BoardAttributes> = q_board.iter().collect();
    let board_id = spawn_board(
        &mut commands,
        &mut meshes,
        &mut color_materials,
        &asset_server,
        &board,
    );
    let move_controls_id = spawn_move_controls(
        &mut commands,
        &mut meshes,
        &mut color_materials,
        &asset_server,
        Vec3::new(0.0, -250.0, 0.0),
        TABLE_TOP_Y,
    );
    children.extend(&[board_id, move_controls_id]);
    commands.entity(table_root).push_children(&children);
    table_updated_event_writer.send(TableUpdated);
}

fn spawn_background(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let shape = Mesh::from(Rectangle::new(800.0, 600.0));
    let color = ColorMaterial::from(Color::rgb(0.3, 0.3, 0.3));
    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);
    let element = (
        WindowBackground,
        MaterialMesh::new(Vec3::new(0.0, 0.0, 0.0), mesh_handle, material_handle),
    );
    commands.spawn(element).id()
}

fn spawn_table_top(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let root = commands
        .spawn(Container::new(Vec3::new(0.0, TABLE_TOP_Y, 1.0)))
        .id();
    let shape = Mesh::from(Ellipse::new(230.0, 110.0));
    let color = ColorMaterial::from(Color::rgb(0.0, 0.0, 0.0));
    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);
    let outer_element = (
        TableTop,
        MaterialMesh::new(Vec3::new(0.0, 0.0, 0.0), mesh_handle, material_handle),
    );
    let outer_element_id = commands.spawn(outer_element).id();

    let border = 20.0;
    let shape = Mesh::from(Ellipse::new(230.0 - border, 110.0 - border));
    let color = ColorMaterial::from(Color::hex("#047804").unwrap());
    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);
    let inner_element = (MaterialMesh::new(
        Vec3::new(0.0, 7.0, 1.0),
        mesh_handle,
        material_handle,
    ),);
    let inner_element_id = commands.spawn(inner_element).id();
    commands
        .entity(outer_element_id)
        .push_children(&[inner_element_id]);
    commands.entity(root).push_children(&[outer_element_id]);
    root
}

fn _spawn_info_message(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    spawn_text(
        commands,
        asset_server,
        Vec3::new(0.0, -280.0, 4.0),
        "Paused. Press space to start.",
        30.0,
        "#eeeeee",
        false,
        InfoMessage,
    )
}

pub fn show_table_ui(mut q_table_root_visibility: Query<&mut Visibility, With<TableRoot>>) {
    let mut table_root_visibility = q_table_root_visibility.single_mut();
    *table_root_visibility = Visibility::Visible;
}
