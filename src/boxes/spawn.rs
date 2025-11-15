use bevy::prelude::*;

enum SpawnItem {
    Bad,
    Good,
}

#[derive(Resource)]
pub struct SpawnList {
    entries: Vec<SpawnItem>,
}

pub(super) fn register(app: &mut App) {}
