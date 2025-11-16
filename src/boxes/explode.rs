use std::time::Duration;

use bevy::{audio::Volume, prelude::*};

use rand::Rng;

use crate::{
    boss_cat::Delay,
    boxes::{BadBox, BoxMadeIt, GameBox, GoodBox},
    character_controls::camera_shake::CameraShake,
    character_controls::{Character, Velocity},
    custom_utils::GameState,
    room::{ROOM_HEIGHT, ROOM_WIDTH},
    ui::loss::LossReason,
};

#[derive(Component)]
pub struct DespawnTimer(pub Timer);

fn box_swatted(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_box: Query<(&Transform, Entity, Has<GoodBox>, Has<BadBox>), With<GameBox>>,
    q_camera: Query<Entity, With<Camera2d>>,
    mut next_state: ResMut<NextState<GameState>>,
    q_player: Query<Entity, With<Character>>,
    e: Res<Explosion>,
    glass: Res<Glass>,
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
                commands.spawn(AudioPlayer::new(e.0.clone()));
                if let Ok(cam_ent) = q_camera.single() {
                    commands
                        .entity(cam_ent)
                        .insert(CameraShake::new(Duration::from_millis(500), 5.0));
                }
            } else if good_box {
                commands.spawn((
                    AudioPlayer::new(glass.0.clone()),
                    PlaybackSettings {
                        volume: Volume::Linear(0.5),
                        ..Default::default()
                    },
                ));
                commands.insert_resource(Delay(Timer::from_seconds(1.0, TimerMode::Once)));
                commands.insert_resource(LossReason::BadKick);
                next_state.set(GameState::BossCatTime);
                commands
                    .entity(q_player.single().expect("no player ;("))
                    .insert(Velocity::default());
            }
            commands.entity(bad_box_entity).despawn();
        }
    }
}

#[derive(Resource)]
struct Explosion(Handle<AudioSource>);

fn load_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Explosion(asset_server.load("sounds/explosion.ogg")));
}

#[derive(Resource)]
struct Cats {
    a: Vec<Handle<Image>>,
}

fn load_cats(mut commands: Commands, rs: Res<AssetServer>) {
    let mut cats = vec![];

    for i in 1..=3 {
        cats.push(rs.load(format!("bad_cat_{i}.png")));
    }

    commands.insert_resource(Cats { a: cats });
}

fn box_made_it_event(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut evr_box_made_it: MessageReader<BoxMadeIt>,
    mut next_state: ResMut<NextState<GameState>>,
    beep: Res<Beep>,
    big: Res<Big>,
    cats: Res<Cats>,
) {
    for box_made_it in evr_box_made_it.read() {
        match box_made_it {
            BoxMadeIt::BadBox => {
                let mut rng = rand::rng();
                let random_number: i32 = rng.random_range(0..3);
                // let file_name = format!("bad_cat_{}.png", random_number);
                commands.spawn((
                    ImageNode {
                        image: cats.a.get(random_number as usize).unwrap().clone(),
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
                commands.insert_resource(LossReason::BadLetThrough);
                commands.spawn((
                    AudioPlayer::new(big.0.clone()),
                    PlaybackSettings {
                        volume: Volume::Linear(0.7),
                        ..Default::default()
                    },
                ));
                next_state.set(GameState::BossCatTime);
            }
            BoxMadeIt::GoodBox => {
                commands.spawn(AudioPlayer::new(beep.0.clone()));
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

#[derive(Resource)]
struct Glass(Handle<AudioSource>);

fn load_glass(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Glass(asset_server.load("sounds/glass_shatter.ogg")));
}

#[derive(Resource)]
struct Big(Handle<AudioSource>);

fn load_big(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Big(asset_server.load("sounds/explosion_large.ogg")));
}

#[derive(Resource)]
struct Beep(Handle<AudioSource>);

fn load_beep(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Beep(asset_server.load("sounds/beep.ogg")));
}

pub(super) fn register(app: &mut App) {
    app.add_systems(
        Startup,
        (load_cats, load_sound, load_glass, load_big, load_beep),
    );
    app.add_systems(Update, (box_swatted, box_made_it_event, despawn_after_time));
}
