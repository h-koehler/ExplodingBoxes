use bevy::prelude::*;

use crate::room::ROOM_HEIGHT;

fn spawn_explosive_decal(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("explosion_residue.png"),
            custom_size: Some(Vec2::new(1000.0, 300.0)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.0, ROOM_HEIGHT as f32 / 2.0 - 150.0, -1.0)),
    ));
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, spawn_explosive_decal);
}
