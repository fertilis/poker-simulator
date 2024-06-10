use bevy::prelude::*;

use super::{
    events::{HeroMoved, TableUpdated},
    resources::DeckResource,
    setup::setup_table,
    states::{HandoutState, PausedState},
    update::{
        attribute_winnings, deal_community_cards, do_showdown, make_move, on_showdown_made,
        pool_moved_chips, start_new_handout,
    },
};

pub struct TablePlugin;

impl Plugin for TablePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PausedState>();
        app.init_state::<HandoutState>();

        app.add_event::<TableUpdated>();
        app.add_event::<HeroMoved>();

        app.init_resource::<DeckResource>();
        app.init_resource::<Time>();

        app.add_systems(Startup, setup_table);

        app.add_systems(
            Update,
            (
                make_move.run_if(in_state(HandoutState::ExpectingMove)),
                pool_moved_chips.run_if(in_state(HandoutState::ExpectingPool)),
                deal_community_cards.run_if(in_state(HandoutState::ExpectingDeal)),
                do_showdown.run_if(in_state(HandoutState::ExpectingShowdown)),
                on_showdown_made.run_if(in_state(HandoutState::ShowdownMade)),
                attribute_winnings.run_if(in_state(HandoutState::ExpectingWinningsAttribution)),
                start_new_handout.run_if(in_state(HandoutState::HandoutEnded)),
            )
                .in_set(TableUpdateSet),
        );

        app.configure_sets(
            Update,
            TableUpdateSet.run_if(in_state(PausedState::Running)),
        );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct TableUpdateSet;
