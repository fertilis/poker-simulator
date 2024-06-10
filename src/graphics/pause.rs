use bevy::{prelude::*, window::PrimaryWindow};

use crate::table::states::PausedState;

use super::{
    base_components::{spawn_button, Container},
    components::{ButtonRect, PauseButton, PauseButtonText},
};

pub fn spawn_pause_button(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    color_materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    pos: Vec3,
) -> Entity {
    let root = commands.spawn(Container::new(pos)).id();
    let button_size = Vec2::new(100.0, 40.0);
    let abs_parent_pos = Vec3::new(-pos.x, -pos.y, 0.0);
    let button = spawn_button(
        commands,
        color_materials,
        meshes,
        asset_server,
        abs_parent_pos.clone(),
        Vec3::new(0.0, 0.0, 0.0),
        button_size.clone(),
        "Start",
        "#777777",
        PauseButton,
        PauseButtonText,
    );
    commands.entity(root).push_children(&[button]);
    root
}

pub fn toggle_pause(
    mut next_state: ResMut<NextState<PausedState>>,
    mut q_text: Query<&mut Text, With<PauseButtonText>>,
    q_button: Query<&ButtonRect, With<PauseButton>>,

    state: Res<State<PausedState>>,
    buttons: Res<ButtonInput<MouseButton>>,
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
            let button_rect = q_button.single().0;
            if button_rect.contains(point) {
                let mut text = q_text.single_mut();
                match state.get() {
                    PausedState::Paused => {
                        next_state.set(PausedState::Running);
                        text.sections[0].value = "Pause".to_string();
                    }
                    PausedState::Running => {
                        next_state.set(PausedState::Paused);
                        text.sections[0].value = "Start".to_string();
                    }
                }
            }
        }
    }
}
