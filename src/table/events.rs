use bevy::prelude::*;

#[derive(Event)]
pub struct TableUpdated;

#[derive(Event)]
pub struct HeroMoved(pub Move);

/// -1 for fold, 0 for check, positive amount for call or raise
pub struct Move(pub i32);
