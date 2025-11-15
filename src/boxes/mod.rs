use std::time::Duration;

use bevy::{color::palettes::css, prelude::*, time::common_conditions::on_timer};
use rand::seq::IndexedRandom;
use walkdir::WalkDir;

use crate::{
    boxes::spawn::{SpawnItem, SpawnList},
    character_controls::{Velocity, swat::Swatted},
    room::{BoxGoal, BoxSpawner, CONVEYOR_SIZE, Movable},
};

pub mod explode;
pub mod spawn;

#[derive(Component)]
pub struct GameBox;

#[derive(Component)]
pub struct BadBox;
#[derive(Component)]
pub struct GoodBox;

const SECONDS_BETWEEN_BOX_SPAWNS: f32 = 1.0;
const BOX_SIZE: f32 = (CONVEYOR_SIZE - 10) as f32;

fn spawn_box(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_box_spawner_transform: Query<&Transform, With<BoxSpawner>>,
    mut boxes_to_spawn: ResMut<SpawnList>,
) {
    for box_spawner_transform in q_box_spawner_transform.iter() {
        if let Some(entry) = boxes_to_spawn.entries.pop() {
            let mut ecmds = commands.spawn((
                GameBox,
                Movable,
                Velocity::default(),
                box_spawner_transform.clone(),
            ));

            match entry {
                SpawnItem::Good(files) => {
                    let good_files = WalkDir::new("./assets/good");
                    let neutral_files = WalkDir::new("./assets/neutral");

                    // let (bevy_path, color) = get_bevy_path(good_files, neutral_files);
                    let path = files.choose(&mut rand::rng()).unwrap();

                    ecmds.insert((
                        GoodBox,
                        Sprite {
                            image: asset_server.load(format!("good/{path}.png")),
                            custom_size: Some(Vec2::new(BOX_SIZE, BOX_SIZE)),
                            // color: color.into(),
                            ..Default::default()
                        },
                    ));
                }
                SpawnItem::Bad(files) => {
                    let bad_files = WalkDir::new("./assets/bad");
                    let neutral_files = WalkDir::new("./assets/neutral");

                    // let (bevy_path, color) = get_bevy_path(bad_files, neutral_files);

                    let path = files.choose(&mut rand::rng()).unwrap();

                    ecmds.insert((
                        BadBox,
                        Sprite {
                            image: asset_server.load(format!("bad/{path}.png")),
                            custom_size: Some(Vec2::new(BOX_SIZE, BOX_SIZE)),
                            // color: color.into(),
                            ..Default::default()
                        },
                    ));
                }
            }
        }
    }
}

fn get_bevy_path(good_files: WalkDir, neutral_files: WalkDir) -> (String, Srgba) {
    let possibilities = good_files
        .into_iter()
        .flatten()
        .map(|x| (x, css::WHITE))
        .chain(neutral_files.into_iter().flatten().map(|x| (x, css::GREEN)))
        .collect::<Vec<_>>();

    let (entry, color) = possibilities
        .choose(&mut rand::rng())
        .expect("no options!!!");

    let mut path_split = entry.path().to_str().unwrap().split("assets/");
    let _ = path_split.next();
    let bevy_path = path_split.next().expect("bad").to_owned();
    (bevy_path, *color)
}

#[derive(Message)]
pub enum BoxMadeIt {
    GoodBox,
    BadBox,
}

fn kill_box(
    mut commands: Commands,
    q_box: Query<(Entity, &Transform, Has<GoodBox>), (With<GameBox>, Without<Swatted>)>,
    q_box_goal_transform: Query<&Transform, With<BoxGoal>>,
    mut evw_box_made_it: MessageWriter<BoxMadeIt>,
) {
    for goal_transform in q_box_goal_transform.iter() {
        let goal_min_x = goal_transform.translation.x - (CONVEYOR_SIZE as f32 / 2.0);
        let goal_max_x = goal_transform.translation.x + (CONVEYOR_SIZE as f32 / 2.0);
        let goal_min_y = goal_transform.translation.y - (CONVEYOR_SIZE as f32 / 2.0);
        let goal_max_y = goal_transform.translation.y + (CONVEYOR_SIZE as f32 / 2.0);
        for (box_entity, box_transform, good_box) in q_box.iter() {
            let x = box_transform.translation.x;
            let y = box_transform.translation.y;
            if x >= goal_min_x && x <= goal_max_x && y >= goal_min_y && y <= goal_max_y {
                evw_box_made_it.write(if good_box {
                    BoxMadeIt::GoodBox
                } else {
                    BoxMadeIt::BadBox
                });
                commands.entity(box_entity).despawn();
            }
        }
    }
}

pub(super) fn register(app: &mut App) {
    explode::register(app);
    spawn::register(app);

    app.add_systems(
        Update,
        (
            spawn_box.run_if(on_timer(Duration::from_secs_f32(
                SECONDS_BETWEEN_BOX_SPAWNS,
            ))),
            kill_box,
        ),
    )
    .add_message::<BoxMadeIt>();
}
