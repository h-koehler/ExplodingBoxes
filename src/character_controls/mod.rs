use bevy::prelude::*;
use crate::room::{ROOM_HEIGHT,ROOM_WIDTH};

const MOVE_SPEED: f32 = 200.0;
const VELOCITY_INCREASE: f32 = 1.0;
const PLAYER_ASS_PATH: &str = "smile.png";
const PLAYER_SIZE: Option<Vec2> = Some(Vec2::new(32.0, 32.0));
const ROOM_INSET: f32 = 4.0;


#[derive(Component)]
pub struct Character;

#[derive(Component, Default)]
pub struct Velocity {
    pub linear_velocity: Vec2,
}

fn player_input(
    inputs: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<&mut Velocity, With<Character>>,
) {
    let mut char_vel = q_player.single_mut().expect("No Player Object");
    let mut dir = Vec2::ZERO;

    if inputs.pressed(KeyCode::KeyW) {
        dir.y += VELOCITY_INCREASE;
    }
    if inputs.pressed(KeyCode::KeyS) {
        dir.y -= VELOCITY_INCREASE;
    }
    if inputs.pressed(KeyCode::KeyA) {
        dir.x -= VELOCITY_INCREASE;
    }
    if inputs.pressed(KeyCode::KeyD) {
        dir.x += VELOCITY_INCREASE;
    }

    char_vel.linear_velocity = dir.normalize_or_zero() * MOVE_SPEED;
}

fn apply_velocity (
    time: Res<Time>, 
    mut q_player: Query<(&mut Transform, &Velocity), With<Character>>,
) {
    let dt = time.delta_secs();
    let (mut trans, vel) = q_player.single_mut().expect("No Player Object");

    trans.translation.x += vel.linear_velocity.x * dt;
    trans.translation.y += vel.linear_velocity.y * dt;

    let half_width = ROOM_WIDTH as f32 / 2.0;
    let half_height = ROOM_HEIGHT as f32 / 2.0;

    let (half_player_width, half_player_height) = if let Some(size) = PLAYER_SIZE {
        (size.x * 0.5, size.y * 0.5)
    }else {
        (50.0, 50.0)
    };

    let min_x = -half_width + half_player_width + ROOM_INSET;
    let max_x = half_width - half_player_width - ROOM_INSET;
    let min_y = -half_height + half_player_height + ROOM_INSET;
    let max_y = half_height - half_player_height - ROOM_INSET;

    trans.translation.x = trans.translation.x.clamp(min_x, max_x);
    trans.translation.y = trans.translation.y.clamp(min_y, max_y);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d::default(),));

    commands.spawn((
        Character,
        Velocity::default(),
        Sprite::from_image(asset_server.load(PLAYER_ASS_PATH)),
    ));
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, (player_input, apply_velocity));
}
