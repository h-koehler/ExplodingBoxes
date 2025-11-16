use bevy::prelude::*;

use crate::{
    boxes::{GameBox, spawn::SpawnList},
    levels::Level,
    ui::win::Win,
};

fn advance_level(
    q_box: Query<(), With<GameBox>>,
    spawn_boxes: Res<SpawnList>,
    mut level: ResMut<Level>,
    mut evw_win: MessageWriter<Win>,
) {
    if !(spawn_boxes.entries.is_empty() && q_box.iter().next().is_none()) {
        return;
    }

    if level.advance() {
        evw_win.write_default();
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(PostUpdate, advance_level);
}
