use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

#[derive(Component)]
pub struct Box;

const BOX_SPAWN_X: f32 = 0.0;
const BOX_SPAWN_Y: f32 = 0.0;
const BOX_SIZE: f32 = 50.0;

fn spawn_box(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Box,
        Sprite {
            image: asset_server.load("green_box.png"),
            custom_size: Some(Vec2::new(BOX_SIZE, BOX_SIZE)),
            ..Default::default()
        },
        Transform::from_xyz(BOX_SPAWN_X, BOX_SPAWN_Y, 0.0),
    ));
}

pub(super) fn register(app: &mut App) {
    app.add_systems(
        Update,
        spawn_box.run_if(on_timer(Duration::from_secs_f32(1.0))),
    );
}
