use bevy::prelude::*;

pub enum SpawnItem {
    Bad(Vec<(&'static str, Srgba)>),
    Good(Vec<(&'static str, Srgba)>),
}

#[derive(Resource, Default)]
pub struct SpawnList {
    pub entries: Vec<SpawnItem>,
}

pub(super) fn register(app: &mut App) {
    app.init_resource::<SpawnList>();
}
