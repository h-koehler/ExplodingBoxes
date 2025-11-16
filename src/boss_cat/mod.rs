use crate::{
    boxes::{BoxKicked, BoxMadeIt},
    character_controls::{Character, Velocity, swat::DidBadSwat},
    custom_utils::GameState,
    room::{Movable, ROOM_HEIGHT, ROOM_WIDTH},
};
use bevy::prelude::*;

const BOSS_ASS_PATH: &str = "boss-cat-angy.png";
const BOSS_SIZE: Vec2 = Vec2::new(200.0, 200.0);
const BOSS_SPAWN_OFFSET: f32 = 50.0;
const BOSS_SPEED: f32 = 120.0;
const TOP_QUARTER_MIN_Y_FACTOR: f32 = 0.25;

#[derive(Component)]
pub struct BossCat;

#[derive(Component)]
enum BossState {
    Entering { target_y: f32, target_x: f32 },
    Talking,
    Exiting,
    Done,
}

#[derive(Resource)]
struct Delay(Timer);

// Boss cat boutta pull up
fn boss_spawning_system(
    mut commands: Commands,
    mut madeit_message_reader: MessageReader<BoxMadeIt>,
    mut kicked_message_reader: MessageReader<BoxKicked>,
) {
    for msg in madeit_message_reader.read() {
        if let BoxMadeIt::BadBox = msg {
            commands.insert_resource(Delay(Timer::from_seconds(1.0, TimerMode::Once)));
        }
    }

    for msg in kicked_message_reader.read() {
        if let BoxKicked::GoodBox = msg {
            commands.insert_resource(Delay(Timer::from_seconds(1.0, TimerMode::Once)));
        }
    }
}

fn boss_movement_system(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &mut BossState), With<BossCat>>,
    q_player: Query<Entity, With<Character>>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    for (mut transform, mut state) in q.iter_mut() {
        match &mut *state {
            BossState::Entering { target_y, target_x } => {
                let y_dir = if transform.translation.y > *target_y {
                    -1.0
                } else if transform.translation.y < *target_y {
                    1.0
                } else {
                    0.0
                };
                transform.translation.y += y_dir * BOSS_SPEED * dt;

                let x_dir = if transform.translation.x > *target_x {
                    -1.0
                } else if transform.translation.x < *target_x {
                    1.0
                } else {
                    0.0
                };

                transform.translation.x += x_dir * BOSS_SPEED * dt;

                if transform.translation.y <= *target_y + 1.0 && transform.translation.x <= *target_x + 1.0 {
                    transform.translation.y = *target_y;
                    transform.translation.x = *target_x;
                    *state = BossState::Talking;
                }
            }
            BossState::Talking => {
                // boss stands still, asserting his dominance
                *state = BossState::Exiting;
            }
            BossState::Exiting => {
                if let Ok(player_ent) = q_player.single() {
                    commands.entity(player_ent).insert(DidBadSwat);
                }
                // boss walks off the screen
                transform.translation.y += BOSS_SPEED * dt;
                let half_h = ROOM_HEIGHT as f32 / 2.0;
                if transform.translation.y > half_h + BOSS_SPAWN_OFFSET {
                    *state = BossState::Done;
                }
            }
            BossState::Done => {}
        }
    }
}

fn boss_cleanup_system(mut commands: Commands, q: Query<(Entity, &BossState), With<BossCat>>) {
    for (entity, state) in q.iter() {
        if let BossState::Done = state {
            commands.entity(entity).despawn();
        }
    }
}

fn boss_delay_spawn_system(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    delay: Option<ResMut<Delay>>,
    mut next_state: ResMut<NextState<GameState>>,
    q_player: Query<(Entity, &Transform), With<Character>>,
) {
    let Some(mut delay) = delay else { return };

    delay.0.tick(time.delta());

    if delay.0.is_finished() {
        commands.remove_resource::<Delay>();
        next_state.set(GameState::BossCatTime);

        let (player_entity, player_transform) = match q_player.single() {
            Ok(t) => t,
            Err(_) => {
                return;
            }
        };
        let player_pos = player_transform.translation;

        let half_w = ROOM_WIDTH as f32 / 2.0;
        let half_h = ROOM_HEIGHT as f32 / 2.0;

        // boss spawning location
        let spawn_x = half_w + BOSS_SPAWN_OFFSET;
        let spawn_y = half_h + BOSS_SPAWN_OFFSET;

        // boss target location
        // let top_quarter_min = ROOM_HEIGHT as f32 * TOP_QUARTER_MIN_Y_FACTOR;
        let target_x = player_pos.x + 80.0;
        let target_y = player_pos.y;
        
        commands.spawn((
            BossCat,
            Movable,
            BossState::Entering { target_y, target_x },
            Sprite {
                image: asset_server.load(BOSS_ASS_PATH),
                custom_size: Some(BOSS_SIZE),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(spawn_x, spawn_y, 5.0)),
        ));

        commands
            .entity(player_entity)
            .insert(Velocity::default());
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(PostUpdate, boss_spawning_system);
    app.add_systems(Update, boss_delay_spawn_system.before(boss_movement_system));
    app.add_systems(Update, boss_movement_system);
    app.add_systems(PostUpdate, boss_cleanup_system);
}
