use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{
    character_controls::{Velocity, swat::Swatted},
    room::{BoxGoal, BoxSpawner, CONVEYOR_SIZE, Movable},
};

pub mod explode;

#[derive(Component)]
pub struct GameBox;

const SECONDS_BETWEEN_BOX_SPAWNS: f32 = 1.0;
const BOX_SIZE: f32 = (CONVEYOR_SIZE - 10) as f32;

fn spawn_box(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_box_spawner_transform: Query<&Transform, With<BoxSpawner>>,
) {
    for box_spawner_transform in q_box_spawner_transform.iter() {
        commands.spawn((
            GameBox,
            Movable,
            Velocity::default(),
            Sprite {
                image: asset_server.load("green_box.png"),
                custom_size: Some(Vec2::new(BOX_SIZE, BOX_SIZE)),
                ..Default::default()
            },
            box_spawner_transform.clone(),
        ));
    }
}

fn kill_box(
    mut commands: Commands,
    q_box: Query<(Entity, &Transform), (With<GameBox>, Without<Swatted>)>,
    q_box_goal_transform: Query<&Transform, With<BoxGoal>>,
) {
    for goal_transform in q_box_goal_transform.iter() {
        let goal_min_x = goal_transform.translation.x - (CONVEYOR_SIZE as f32 / 2.0);
        let goal_max_x = goal_transform.translation.x + (CONVEYOR_SIZE as f32 / 2.0);
        let goal_min_y = goal_transform.translation.y - (CONVEYOR_SIZE as f32 / 2.0);
        let goal_max_y = goal_transform.translation.y + (CONVEYOR_SIZE as f32 / 2.0);
        for (box_entity, box_transform) in q_box.iter() {
            let x = box_transform.translation.x;
            let y = box_transform.translation.y;
            if x >= goal_min_x && x <= goal_max_x && y >= goal_min_y && y <= goal_max_y {
                commands.entity(box_entity).despawn();
            }
        }
    }
}

pub(super) fn register(app: &mut App) {
    explode::register(app);

    app.add_systems(
        Update,
        (
            spawn_box.run_if(on_timer(Duration::from_secs_f32(
                SECONDS_BETWEEN_BOX_SPAWNS,
            ))),
            kill_box,
        ),
    );
}
