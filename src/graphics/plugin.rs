use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use super::{
    board::update_board,
    components::MainCamera,
    events::InfoMessageEvent,
    move_controls::{handle_clicks_on_move_buttons, update_move_controls},
    pause::toggle_pause,
    player::update_players,
    table_ui::{setup_table_ui, show_table_ui},
};
use crate::table::{self, events::TableUpdated};

pub struct TableUiPlugin;

impl Plugin for TableUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InfoMessageEvent>();

        app.init_resource::<ButtonInput<MouseButton>>();

        app.add_systems(
            Startup,
            (
                spawn_camera,
                setup_table_ui.after(table::setup_table),
                show_table_ui.after(setup_table_ui),
            ),
        );
        app.add_systems(
            Update,
            (
                toggle_pause,
                handle_clicks_on_move_buttons,
                update_move_controls,
                update_board,
                update_players,
                emit_table_updated_event.run_if(on_timer(Duration::from_millis(100))),
            )
                .chain(),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn emit_table_updated_event(mut table_updated_event_writer: EventWriter<TableUpdated>) {
    table_updated_event_writer.send(TableUpdated);
}
