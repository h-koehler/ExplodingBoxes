use bevy::{audio::Volume, prelude::*};

use rand::Rng;

use crate::{
    boxes::{BadBox, BoxMadeIt, GameBox, GoodBox},
    custom_utils::GameState,
    room::{ROOM_HEIGHT, ROOM_WIDTH},
};

#[derive(Component)]
pub struct DespawnTimer(pub Timer);

fn box_swatted(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_box: Query<(&Transform, Entity, Has<GoodBox>, Has<BadBox>), With<GameBox>>,
) {
    // Doubled to delay sound effects and avoid despawning boxes on the screen.
    let min_x = -(ROOM_WIDTH as f32);
    let max_x = ROOM_WIDTH as f32;
    let min_y = -(ROOM_HEIGHT as f32);
    let max_y = ROOM_HEIGHT as f32;
    for (bad_box_transform, bad_box_entity, good_box, bad_box) in q_box.iter() {
        let x = bad_box_transform.translation.x;
        let y = bad_box_transform.translation.y;
        if x < min_x || x > max_x || y < min_y || y > max_y {
            if bad_box {
                let mut rng = rand::rng();
                let random_number: i32 = rng.random_range(1..=3);
                let sound_file_name = format!("sounds/explosion_{}.ogg", random_number);
                commands.spawn(AudioPlayer::new(asset_server.load(sound_file_name)));
            } else if good_box {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("sounds/glass_shatter.ogg")),
                    PlaybackSettings {
                        volume: Volume::Linear(0.5),
                        ..Default::default()
                    },
                ));
            }
            commands.entity(bad_box_entity).despawn();
        }
    }
}

fn box_made_it_event(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut evr_box_made_it: MessageReader<BoxMadeIt>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for box_made_it in evr_box_made_it.read() {
        match box_made_it {
            BoxMadeIt::BadBox => {
                let mut rng = rand::rng();
                let random_number: i32 = rng.random_range(1..=3);
                let file_name = format!("bad_cat_{}.png", random_number);
                commands.spawn((
                    ImageNode {
                        image: asset_server.load(file_name),
                        ..Default::default()
                    },
                    GlobalZIndex(100),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..Default::default()
                    },
                    DespawnTimer(Timer::from_seconds(1.0, TimerMode::Once)),
                ));
                commands.spawn((
                    AudioPlayer::new(asset_server.load("sounds/explosion_large.ogg")),
                    PlaybackSettings {
                        volume: Volume::Linear(0.7),
                        ..Default::default()
                    },
                ));
                next_state.set(GameState::BossCatTime);
            }
            BoxMadeIt::GoodBox => {
                commands.spawn(AudioPlayer::new(asset_server.load("sounds/beep.ogg")));
            }
        }
    }
}

fn despawn_after_time(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DespawnTimer)>,
) {
    for (entity, mut despawn_timer) in query.iter_mut() {
        despawn_timer.0.tick(time.delta());
        if despawn_timer.0.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Update, (box_swatted, box_made_it_event, despawn_after_time));
}
