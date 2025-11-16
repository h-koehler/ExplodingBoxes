use bevy::prelude::*;

use crate::{
    boxes::{GameBox, spawn::SpawnList},
    levels::Level,
};

fn advance_level(
    q_box: Query<(), With<GameBox>>,
    spawn_boxes: Res<SpawnList>,
    mut level: ResMut<Level>,
) {
    if !(spawn_boxes.entries.is_empty() && q_box.iter().next().is_none()) {
        return;
    }

    if level.advance() {
        println!("YOU DID IT :DDDDD");
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Update, advance_level);
}
