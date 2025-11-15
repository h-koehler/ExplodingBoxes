use bevy::prelude::*;

pub enum SpawnItem {
    Bad(Vec<&'static str>),
    Good(Vec<&'static str>),
}

#[derive(Resource, Default)]
pub struct SpawnList {
    pub entries: Vec<SpawnItem>,
}

pub(super) fn register(app: &mut App) {
    app.init_resource::<SpawnList>();
}
