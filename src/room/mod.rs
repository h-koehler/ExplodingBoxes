use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};

#[derive(Component)]
pub struct Movable;

#[derive(Component)]
struct Conveyor {
    pub direction: Vec2,
}

pub const ROOM_HEIGHT: u32 = 700;
pub const ROOM_WIDTH: u32 = 1100;

const CONVEYOR_SIZE: u32 = 50;

const X_OFFSET: f32 = -(ROOM_WIDTH as f32 / 2.0) + (CONVEYOR_SIZE as f32 / 2.0);
const Y_OFFSET: f32 = ROOM_HEIGHT as f32 / 2.0 - (CONVEYOR_SIZE as f32 / 2.0);

fn setup_window_resolution(mut q_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut win = q_window.single_mut().unwrap();
    win.resolution = WindowResolution::new(ROOM_WIDTH, ROOM_HEIGHT);
}

fn setup_room(mut commands: Commands, asset_server: Res<AssetServer>) {
    create_line(&mut commands, &asset_server, 1, 3, Vec2::X, 1);

    create_conveyor(
        &mut commands,
        &asset_server,
        ROOM_WIDTH / CONVEYOR_SIZE - 2,
        4,
        Vec2::NEG_Y,
    );
    create_conveyor(
        &mut commands,
        &asset_server,
        ROOM_WIDTH / CONVEYOR_SIZE - 2,
        5,
        Vec2::NEG_Y,
    );

    create_line(&mut commands, &asset_server, 1, 6, Vec2::NEG_X, 1);

    create_conveyor(
        &mut commands,
        &asset_server,
        ROOM_WIDTH / CONVEYOR_SIZE - 2,
        4,
        Vec2::NEG_Y,
    );
    create_conveyor(
        &mut commands,
        &asset_server,
        ROOM_WIDTH / CONVEYOR_SIZE - 2,
        5,
        Vec2::NEG_Y,
    );

    create_line(&mut commands, &asset_server, 1, 9, Vec2::X, 1);
}

fn create_conveyor(
    commands: &mut Commands,
    asset_server: &AssetServer,
    x: u32,
    y: u32,
    direction: Vec2,
) {
    commands.spawn((
        Sprite {
            image: asset_server.load("smile.png"),
            custom_size: Some(Vec2::new(CONVEYOR_SIZE as f32, CONVEYOR_SIZE as f32)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(
            X_OFFSET + (x * CONVEYOR_SIZE) as f32,
            Y_OFFSET - (y * CONVEYOR_SIZE) as f32,
            1.0,
        )),
        Conveyor { direction },
    ));
}

fn create_line(
    commands: &mut Commands,
    asset_server: &AssetServer,
    start_x: u32,
    y: u32,
    direction: Vec2,
    x_dir: i32,
) {
    for x in start_x..(ROOM_WIDTH / CONVEYOR_SIZE - start_x) {
        create_conveyor(
            commands,
            asset_server,
            (x_dir * x as i32) as u32,
            y,
            direction,
        );
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, (setup_room, setup_window_resolution));
}
