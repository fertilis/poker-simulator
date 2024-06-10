use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum HandoutState {
    #[default]
    ExpectingMove,
    ExpectingPool,
    ExpectingDeal,
    ExpectingShowdown,
    ShowdownMade,
    ExpectingWinningsAttribution,
    HandoutEnded,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PausedState {
    Paused,
    #[default]
    Running,
}
