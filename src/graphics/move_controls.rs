use bevy::{prelude::*, window::PrimaryWindow};

use crate::table::{
    components::{Amount, Chips, MovedChips, Player, PlayerInAction, PlayerIsHero, Pot, Street},
    events::{HeroMoved, Move, TableUpdated},
};

use super::{
    base_components::{spawn_button, Container},
    components::{
        ButtonRect, CallButtonText, DummyLabel, FoldButton, MoveButtons, MoveControls,
        RaiseButtonText,
    },
};

use crate::table::betting;

pub fn spawn_move_controls(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    color_materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    pos: Vec3,
    y_offset: f32,
) -> Entity {
    let root = commands.spawn((MoveControls, Container::new(pos))).id();
    let button_size = Vec2::new(130.0, 40.0);
    let abs_parent_pos = Vec3::new(-pos.x, -pos.y - y_offset, 0.0);
    let fold_button = spawn_button(
        commands,
        color_materials,
        meshes,
        asset_server,
        abs_parent_pos.clone(),
        Vec3::new(-140.0, 0.0, 0.0),
        button_size.clone(),
        "Fold",
        "#3469ba",
        (FoldButton, MoveButtons::FoldButton),
        DummyLabel,
    );
    let call_button = spawn_button(
        commands,
        color_materials,
        meshes,
        asset_server,
        abs_parent_pos.clone(),
        Vec3::new(0.0, 0.0, 0.0),
        button_size.clone(),
        "Call",
        "#047804",
        MoveButtons::CallButton(Chips(0)),
        CallButtonText,
    );
    let raise_button = spawn_button(
        commands,
        color_materials,
        meshes,
        asset_server,
        abs_parent_pos.clone(),
        Vec3::new(140.0, 0.0, 0.0),
        button_size.clone(),
        "Raise",
        "#b81600",
        MoveButtons::RaiseButton(Chips(0)),
        RaiseButtonText,
    );
    commands
        .entity(root)
        .push_children(&[fold_button, call_button, raise_button]);
    root
}

pub fn handle_clicks_on_move_buttons(
    mut hero_moved_event_writer: EventWriter<HeroMoved>,
    buttons: Res<ButtonInput<MouseButton>>,
    q_buttons: Query<(&ButtonRect, &MoveButtons)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = q_windows.single();
        let window_width = window.width();
        let window_height = window.height();
        if let Some(cursor_position) = window.cursor_position() {
            let point = Vec2::new(
                cursor_position.x - window_width / 2.0,
                cursor_position.y - window_height / 2.0,
            );
            for (button_rect, move_button) in q_buttons.iter() {
                if button_rect.0.contains(point) {
                    match move_button {
                        MoveButtons::FoldButton => {
                            hero_moved_event_writer.send(HeroMoved(Move(-1)));
                        }
                        MoveButtons::CallButton(chips) => {
                            hero_moved_event_writer.send(HeroMoved(Move(chips.0 as i32)));
                        }
                        MoveButtons::RaiseButton(chips) => {
                            hero_moved_event_writer.send(HeroMoved(Move(chips.0 as i32)));
                        }
                    }
                }
            }
        }
    }
}

pub fn update_move_controls(
    mut table_updated_event_reader: EventReader<TableUpdated>,
    mut q_visibilities: ParamSet<(
        Query<&mut Visibility, With<MoveControls>>,
        Query<&mut Visibility, With<FoldButton>>,
    )>,
    mut q_texts: ParamSet<(
        Query<&mut Text, With<RaiseButtonText>>,
        Query<&mut Text, With<CallButtonText>>,
    )>,
    mut q_move_buttons: Query<&mut MoveButtons>,
    q_hero_in_action: Query<Entity, (With<Player>, With<PlayerIsHero>, With<PlayerInAction>)>,
    q_players: Query<betting::PlayerAttributes, With<Player>>,
    q_pot: Query<&Pot>,
    q_street: Query<&Street>,
) {
    let table_updated: bool = table_updated_event_reader.read().last().is_some();
    if !table_updated {
        return;
    }

    let mut q_move_controls_visibility = q_visibilities.p0();
    let mut move_controls_visibility = q_move_controls_visibility.single_mut();
    let hero_in_action: bool = q_hero_in_action.iter().last().is_some();
    if hero_in_action {
        *move_controls_visibility = Visibility::Visible;
        let players = q_players.iter().collect::<Vec<_>>();
        let betting = betting::Betting::new(&players, q_pot.single().amount(), q_street.single().0);
        let move_options = betting.move_options();
        let call_option = move_options[move_options.len() - 2];
        let raise_option = move_options[move_options.len() - 1];
        let tocall = betting.tocall();
        if tocall == 0 {
            let mut q_fold_button_visibility = q_visibilities.p1();
            let mut fold_button_visibility = q_fold_button_visibility.single_mut();
            *fold_button_visibility = Visibility::Hidden;

            let mut q_call_button_text = q_texts.p1();
            let mut call_button_text = q_call_button_text.single_mut();
            call_button_text.sections[0].value = "Check".to_string();
        } else {
            let mut q_fold_button_visibility = q_visibilities.p1();
            let mut fold_button_visibility = q_fold_button_visibility.single_mut();
            *fold_button_visibility = Visibility::Visible;

            let mut q_call_button_text = q_texts.p1();
            let mut call_button_text = q_call_button_text.single_mut();
            call_button_text.sections[0].value = "Call".to_string();
        }
        for mut move_button in q_move_buttons.iter_mut() {
            match *move_button {
                MoveButtons::FoldButton => {}
                MoveButtons::CallButton(_) => {
                    *move_button = MoveButtons::CallButton(Chips(call_option as u32));
                }
                MoveButtons::RaiseButton(_) => {
                    *move_button = MoveButtons::RaiseButton(Chips(raise_option as u32));
                }
            }
        }
        let mut q_raise_button_text = q_texts.p0();
        let mut raise_button_text = q_raise_button_text.single_mut();
        raise_button_text.sections[0].value = format!(
            "Raise {}",
            MovedChips(Chips(raise_option as u32)).amount_str()
        );
    } else {
        *move_controls_visibility = Visibility::Hidden;
        let mut q_fold_button_visibility = q_visibilities.p1();
        let mut fold_button_visibility = q_fold_button_visibility.single_mut();
        *fold_button_visibility = Visibility::Hidden;
    }
}
