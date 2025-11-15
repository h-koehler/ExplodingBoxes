use std::arch::x86_64::_MM_CMPINT_FALSE;

use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};

#[derive(Component)]
pub struct Movable;

#[derive(Component)]
pub struct Conveyor {
    pub direction: Vec2,
    pub corner_direction: Option<Vec2>,
}

#[derive(Component)]
pub struct BoxSpawner;

#[derive(Component)]
pub struct BoxGoal;

pub const ROOM_HEIGHT: u32 = 700;
pub const ROOM_WIDTH: u32 = 1100;

pub const CONVEYOR_SIZE: u32 = 50;
pub const CONVEYOR_SPEED: f32 = 100.0;

pub const X_OFFSET: f32 = -(ROOM_WIDTH as f32 / 2.0) + (CONVEYOR_SIZE as f32 / 2.0);
pub const Y_OFFSET: f32 = ROOM_HEIGHT as f32 / 2.0 - (CONVEYOR_SIZE as f32 / 2.0);

fn setup_window_resolution(mut q_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut win = q_window.single_mut().unwrap();
    win.resolution = WindowResolution::new(ROOM_WIDTH, ROOM_HEIGHT);
}

fn setup_room(mut commands: Commands, asset_server: Res<AssetServer>) {
    // First conveyor is the box spawner.
    let mut conveyor_commands = create_conveyor(&mut commands, &asset_server, 1, 3, Vec2::X, None);
    conveyor_commands.insert(BoxSpawner);

    commands.spawn((
        Name::new("Background"),
        Sprite {
            custom_size: Some(Vec2::new(ROOM_WIDTH as f32, ROOM_HEIGHT as f32)),
            image: asset_server.load("background.png"),
            ..Default::default()
        },
        Transform::from_translation(Vec3::NEG_Z * 10.0),
    ));

    create_line(
        &mut commands,
        &asset_server,
        2,
        1,
        3,
        Vec2::X,
        true,
        Vec2::X,
        Vec2::NEG_Y,
    );

    create_conveyor(
        &mut commands,
        &asset_server,
        ROOM_WIDTH / CONVEYOR_SIZE - 2,
        4,
        Vec2::NEG_Y,
        None,
    );
    create_conveyor(
        &mut commands,
        &asset_server,
        ROOM_WIDTH / CONVEYOR_SIZE - 2,
        5,
        Vec2::NEG_Y,
        None,
    );

    create_line(
        &mut commands,
        &asset_server,
        1,
        1,
        6,
        Vec2::NEG_X,
        false,
        Vec2::NEG_Y,
        Vec2::NEG_Y,
    );

    create_conveyor(&mut commands, &asset_server, 1, 7, Vec2::NEG_Y, None);
    create_conveyor(&mut commands, &asset_server, 1, 8, Vec2::NEG_Y, None);

    create_line(
        &mut commands,
        &asset_server,
        1,
        2,
        9,
        Vec2::X,
        true,
        Vec2::NEG_Y,
        Vec2::X,
    );

    // Last conveyor is the box goal.
    let mut conveyor_commands = create_conveyor(
        &mut commands,
        &asset_server,
        ROOM_WIDTH / CONVEYOR_SIZE - 2,
        9,
        Vec2::X,
        None,
    );
    conveyor_commands.insert(BoxGoal);
}

fn create_conveyor<'a>(
    commands: &'a mut Commands,
    asset_server: &AssetServer,
    x: u32,
    y: u32,
    direction: Vec2,
    corner_direction: Option<Vec2>,
) -> EntityCommands<'a> {
    commands.spawn((
        Sprite {
            image: asset_server.load("conveyer_piece.png"),
            custom_size: Some(Vec2::new(CONVEYOR_SIZE as f32, CONVEYOR_SIZE as f32)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(
            X_OFFSET + (x * CONVEYOR_SIZE) as f32,
            Y_OFFSET - (y * CONVEYOR_SIZE) as f32,
            0.0,
        )),
        Conveyor {
            direction,
            corner_direction,
        },
    ))
}

fn create_line(
    commands: &mut Commands,
    asset_server: &AssetServer,
    start_x_offset: u32,
    end_x_offset: u32,
    y: u32,
    direction: Vec2,
    positive_x_dir: bool,
    first_dir: Vec2,
    last_dir: Vec2,
) {
    let n = ROOM_WIDTH / CONVEYOR_SIZE - end_x_offset;
    for x in start_x_offset..n {
        // Blame corns for this.
        let (dir, after_turn_dir) = if x == start_x_offset && first_dir != direction {
            if positive_x_dir {
                (first_dir, Some(direction))
            } else {
                (direction, Some(first_dir))
            }
        } else if x == n - 1 && last_dir != direction {
            if positive_x_dir {
                (direction, Some(last_dir))
            } else {
                (last_dir, Some(direction))
            }
        } else {
            (direction, None)
        };

        create_conveyor(commands, asset_server, x, y, dir, after_turn_dir);
    }
}

fn move_thing_on_conveyor(
    time: Res<Time>,
    mut q_trans: Query<(&mut Transform, &Sprite), (With<Movable>, Without<Conveyor>)>,
    q_conveyor: Query<(&Transform, &Conveyor), Without<Movable>>,
) {
    for (mut trans, moving_thing) in q_trans.iter_mut() {
        let Some((_, conv, t)) = q_conveyor
            .iter()
            .map(|(t, c)| (t.translation.distance_squared(trans.translation), c, t))
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        else {
            continue;
        };

        let rect = Rect {
            min: Vec2::new(
                t.translation.x - CONVEYOR_SIZE as f32 / 2.0,
                t.translation.y - CONVEYOR_SIZE as f32 / 2.0,
            ),
            max: Vec2::new(
                t.translation.x + CONVEYOR_SIZE as f32 / 2.0,
                t.translation.y + CONVEYOR_SIZE as f32 / 2.0,
            ),
        };

        let this_size = moving_thing
            .custom_size
            .expect("no custom size set. it's joever");

        let this_rect = Rect {
            min: Vec2::new(
                trans.translation.x - this_size.x as f32 / 2.0,
                trans.translation.y - this_size.y as f32 / 2.0,
            ),
            max: Vec2::new(
                trans.translation.x + this_size.x as f32 / 2.0,
                trans.translation.y + this_size.y as f32 / 2.0,
            ),
        };

        if rects_overlap(&this_rect, &rect) {
            let direction = if let Some(corner_direction) = conv.corner_direction {
                // Corner conveyor. Not for the faint of heart.
                let positive_direction = conv.direction == Vec2::X || conv.direction == Vec2::Y;
                let positive_corner_direction =
                    corner_direction == Vec2::X || corner_direction == Vec2::Y;
                let positive_diagonal = positive_direction && !positive_corner_direction
                    || !positive_direction && positive_corner_direction;
                let x = trans.translation.x - rect.min.x;
                let y = trans.translation.y - rect.min.y;
                if positive_diagonal && x < y
                    || !positive_diagonal && x + y > (CONVEYOR_SIZE as f32)
                {
                    conv.direction
                } else {
                    corner_direction
                }
            } else {
                // Straight conveyor.
                conv.direction
            };
            trans.translation +=
                Vec3::new(direction.x, direction.y, 0.0) * time.delta_secs() * CONVEYOR_SPEED;
        }
    }
}

fn rects_overlap(a: &Rect, b: &Rect) -> bool {
    // Separating Axis Theorem for AABBs:
    // if one is strictly to the left/right or above/below the other, no collision
    !(a.max.x <= b.min.x || // a is left of b
      a.min.x >= b.max.x || // a is right of b
      a.max.y <= b.min.y || // a is below b
      a.min.y >= b.max.y) // a is above b
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, (setup_room, setup_window_resolution));

    app.add_systems(Update, move_thing_on_conveyor);
}
