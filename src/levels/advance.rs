use bevy::{audio::Volume, prelude::*};

use crate::{
    boxes::{GameBox, spawn::SpawnList},
    levels::Level,
    ui::win::Win,
};

#[derive(Resource)]
// evil hack
pub struct JustReset;

fn advance_level(
    q_box: Query<(), With<GameBox>>,
    spawn_boxes: Res<SpawnList>,
    mut level: ResMut<Level>,
    mut evw_win: MessageWriter<Win>,
    just_reset: Option<Res<JustReset>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.remove_resource::<JustReset>();
    if just_reset.is_some() {
        return;
    }
    if !(spawn_boxes.entries.is_empty() && q_box.iter().next().is_none()) {
        return;
    }

    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/level_up.ogg")),
        PlaybackSettings {
            volume: Volume::Linear(0.6),
            ..Default::default()
        },
    ));

    if level.advance() {
        evw_win.write_default();
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(PostUpdate, advance_level);
}
