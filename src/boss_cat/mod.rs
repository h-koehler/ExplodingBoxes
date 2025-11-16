use crate::{
    boxes::{BoxKicked, BoxMadeIt}, custom_utils::GameState, room::{Movable, ROOM_HEIGHT}
};
use bevy::prelude::*;

const BOSS_ASS_PATH: &str = "green_box.png";
const BOSS_SIZE: Vec2 = Vec2::new(50.0, 50.0);
const BOSS_SPAWN_OFFSET: f32 = 50.0;
const BOSS_SPEED: f32 = 120.0;
const TOP_QUARTER_MIN_Y_FACTOR: f32 = 0.25;

#[derive(Component)]
pub struct BossCat;

#[derive(Component)]
enum BossState {
    Entering { target_y: f32 },
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
) {
    let dt = time.delta_secs();
    for (mut transform, mut state) in q.iter_mut() {
        match &mut *state {
            BossState::Entering { target_y } => {
                let dir = if transform.translation.y > *target_y {
                    -1.0
                } else {
                    0.0
                };
                transform.translation.y += dir * BOSS_SPEED * dt;

                if transform.translation.y <= *target_y + 1.0 {
                    transform.translation.y = *target_y;
                    *state = BossState::Talking;
                }
            }
            BossState::Talking => {
                // boss stands still, asserting his dominance
                *state = BossState::Exiting;
            }
            BossState::Exiting => {
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
) {
    let Some(mut delay) = delay else { return };

    delay.0.tick(time.delta());

    if delay.0.is_finished() {
        commands.remove_resource::<Delay>();
        next_state.set(GameState::BossCatTime);

        // --- spawn boss here ---
        let half_h = ROOM_HEIGHT as f32 / 2.0;
        let spawn_y = half_h + BOSS_SPAWN_OFFSET;

        let top_quarter_min = ROOM_HEIGHT as f32 * TOP_QUARTER_MIN_Y_FACTOR;
        let target_y = (top_quarter_min + half_h) * 0.5;

        commands.spawn((
            BossCat,
            Movable,
            BossState::Entering { target_y },
            Sprite {
                image: asset_server.load(BOSS_ASS_PATH),
                custom_size: Some(BOSS_SIZE),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(0.0, spawn_y, 5.0)),
        ));
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(PostUpdate, boss_spawning_system);
    app.add_systems(Update, boss_delay_spawn_system.before(boss_movement_system));
    app.add_systems(Update, boss_movement_system);
    app.add_systems(PostUpdate, boss_cleanup_system);
}
