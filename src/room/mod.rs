use bevy::prelude::*;

use crate::{custom_utils::GameState, ui::UI_HEIGHT};

#[derive(Component)]
pub struct Movable;

#[derive(Component)]
pub struct Conveyor {
    pub direction: Vec2,
    pub after_turn_direction: Option<Vec2>,
}

#[derive(Component)]
pub struct BoxSpawner;

#[derive(Component)]
pub struct BoxGoal;

pub enum ConveyorLayout {
    Snake3,
    Snake4,
    Line1,
    Line2,
    Spiral,
    Test,
}

pub const ROOM_HEIGHT: u32 = 700;
pub const ROOM_WIDTH: u32 = 1100;

pub const CONVEYOR_LAYOUT: ConveyorLayout = ConveyorLayout::Snake3;
pub const CONVEYOR_SIZE: u32 = 50;
pub const CONVEYOR_SPEED: f32 = 100.0;

pub const ROOM_CONVEYOR_WIDTH: u32 = ROOM_WIDTH / CONVEYOR_SIZE;
pub const ROOM_CONVEYOR_HEIGHT: u32 = ROOM_HEIGHT / CONVEYOR_SIZE;

pub const X_OFFSET: f32 = -(ROOM_WIDTH as f32 / 2.0) + (CONVEYOR_SIZE as f32 / 2.0);
pub const Y_OFFSET: f32 = ROOM_HEIGHT as f32 / 2.0 - (CONVEYOR_SIZE as f32 / 2.0) + UI_HEIGHT;

fn setup_room(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Background"),
        Sprite {
            custom_size: Some(Vec2::new(
                ROOM_WIDTH as f32 + 20.0,
                ROOM_HEIGHT as f32 + 20.0,
            )),
            image: asset_server.load("background.png"),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.0, UI_HEIGHT / 2.0, -10.0)),
    ));

    create_conveyor_layout(CONVEYOR_LAYOUT, &mut commands, &asset_server);
}

fn create_conveyor_layout(
    layout: ConveyorLayout,
    commands: &mut Commands,
    asset_server: &AssetServer,
) {
    match layout {
        ConveyorLayout::Snake3 => snake_3(commands, asset_server),
        ConveyorLayout::Snake4 => snake_4(commands, asset_server),
        ConveyorLayout::Line1 => line_1(9, commands, asset_server),
        ConveyorLayout::Line2 => line_2(commands, asset_server),
        ConveyorLayout::Spiral => spiral(commands, asset_server),
        ConveyorLayout::Test => test(commands, asset_server),
    }
}

fn create_conveyor<'a>(
    commands: &'a mut Commands,
    asset_server: &AssetServer,
    x: u32,
    y: u32,
    direction: Vec2,
    after_turn_direction: Option<Vec2>,
) -> EntityCommands<'a> {
    let rotation_direction = match after_turn_direction {
        Some(atd) => atd,
        None => direction,
    };
    commands.spawn((
        Sprite {
            image: asset_server.load("conveyor_0.png"),
            custom_size: Some(Vec2::new(CONVEYOR_SIZE as f32, CONVEYOR_SIZE as f32)),
            ..Default::default()
        },
        Transform::default()
            .with_rotation(Quat::from_axis_angle(
                Vec3::Z,
                rotation_direction.y.atan2(rotation_direction.x),
            ))
            .with_translation(Vec3::new(
                X_OFFSET + (x * CONVEYOR_SIZE) as f32,
                Y_OFFSET - (y * CONVEYOR_SIZE) as f32,
                0.0,
            )),
        Conveyor {
            direction,
            after_turn_direction,
        },
    ))
}

fn create_conveyor_start(
    commands: &mut Commands,
    asset_server: &AssetServer,
    x: u32,
    y: u32,
    direction: Vec2,
    after_turn_direction: Option<Vec2>,
) {
    commands.spawn((
        Sprite {
            image: asset_server.load("conveyor_end.png"),
            custom_size: Some(Vec2::new(
                1.5 * CONVEYOR_SIZE as f32,
                1.5 * CONVEYOR_SIZE as f32,
            )),
            ..Default::default()
        },
        Transform::default().with_translation(Vec3::new(
            X_OFFSET + (x * CONVEYOR_SIZE) as f32,
            Y_OFFSET - (y * CONVEYOR_SIZE) as f32,
            2.9,
        )),
    ));
    let mut conveyor_entity = create_conveyor(
        commands,
        asset_server,
        x,
        y,
        direction,
        after_turn_direction,
    );
    conveyor_entity.insert(BoxSpawner);
}

fn create_conveyor_end(
    commands: &mut Commands,
    asset_server: &AssetServer,
    x: u32,
    y: u32,
    direction: Vec2,
    after_turn_direction: Option<Vec2>,
) {
    commands.spawn((
        Sprite {
            image: asset_server.load("conveyor_end.png"),
            custom_size: Some(Vec2::new(
                1.5 * CONVEYOR_SIZE as f32,
                1.5 * CONVEYOR_SIZE as f32,
            )),
            ..Default::default()
        },
        Transform::default().with_translation(Vec3::new(
            X_OFFSET + (x * CONVEYOR_SIZE) as f32,
            Y_OFFSET - (y * CONVEYOR_SIZE) as f32,
            2.9,
        )),
    ));
    let mut conveyor_entity = create_conveyor(
        commands,
        asset_server,
        x,
        y,
        direction,
        after_turn_direction,
    );
    conveyor_entity.insert(BoxGoal);
}

fn create_line_x(
    commands: &mut Commands,
    asset_server: &AssetServer,
    start_x: u32,
    end_x: u32,
    y: u32,
    direction: Vec2,
    start_dir: Vec2,
) {
    let min_x = start_x.min(end_x);
    let max_x = start_x.max(end_x);
    for x in min_x..=max_x {
        let (dir, after_turn_dir) = if x == start_x && start_dir != direction {
            (start_dir, Some(direction))
        } else {
            (direction, None)
        };
        create_conveyor(commands, asset_server, x, y, dir, after_turn_dir);
    }
}

fn create_line_y(
    commands: &mut Commands,
    asset_server: &AssetServer,
    start_y: u32,
    end_y: u32,
    x: u32,
    direction: Vec2,
    start_dir: Vec2,
) {
    let min_y = start_y.min(end_y);
    let max_y = start_y.max(end_y);
    for y in min_y..=max_y {
        let (dir, after_turn_dir) = if y == start_y && start_dir != direction {
            (start_dir, Some(direction))
        } else {
            (direction, None)
        };
        create_conveyor(commands, asset_server, x, y, dir, after_turn_dir);
    }
}

fn move_thing_on_conveyor(
    time: Res<Time>,
    mut q_movable: Query<(&mut Transform, &Sprite), (With<Movable>, Without<Conveyor>)>,
    q_conveyor: Query<(&Transform, &Conveyor), Without<Movable>>,
) {
    for (mut movable_transform, moving_sprite) in q_movable.iter_mut() {
        let Some((_, conveyor, conveyor_transform)) = q_conveyor
            .iter()
            .map(|(t, c)| {
                (
                    t.translation
                        .distance_squared(movable_transform.translation),
                    c,
                    t,
                )
            })
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        else {
            continue;
        };

        let conveyor_rect = Rect {
            min: Vec2::new(
                conveyor_transform.translation.x - CONVEYOR_SIZE as f32 / 2.0,
                conveyor_transform.translation.y - CONVEYOR_SIZE as f32 / 2.0,
            ),
            max: Vec2::new(
                conveyor_transform.translation.x + CONVEYOR_SIZE as f32 / 2.0,
                conveyor_transform.translation.y + CONVEYOR_SIZE as f32 / 2.0,
            ),
        };

        let movable_size = moving_sprite
            .custom_size
            .expect("no custom size set. it's joever");

        let movable_rect = Rect {
            min: Vec2::new(
                movable_transform.translation.x - movable_size.x as f32 / 2.0,
                movable_transform.translation.y - movable_size.y as f32 / 2.0,
            ),
            max: Vec2::new(
                movable_transform.translation.x + movable_size.x as f32 / 2.0,
                movable_transform.translation.y + movable_size.y as f32 / 2.0,
            ),
        };

        if rects_overlap(&movable_rect, &conveyor_rect) {
            let direction = if let Some(after_turn_direction) = conveyor.after_turn_direction {
                // Computes which triangle the movable thing is in.
                let translation_from_conveyor_center =
                    (movable_transform.translation - conveyor_transform.translation).truncate();
                let before_turn = translation_from_conveyor_center.dot(-conveyor.direction);
                let after_turn = translation_from_conveyor_center.dot(after_turn_direction);
                if before_turn > after_turn {
                    conveyor.direction
                } else {
                    after_turn_direction
                }
            } else {
                // Straight conveyor.
                conveyor.direction
            };
            movable_transform.translation +=
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

#[derive(Resource)]
struct ConveyorSprites {
    frames: Vec<Handle<Image>>,
}

const SPEED: f32 = 22.0;

fn animate_conveyors(
    sprites: Res<ConveyorSprites>,
    time: Res<Time>,
    mut q_conveyor: Query<&mut Sprite, With<Conveyor>>,
) {
    for mut s in q_conveyor.iter_mut() {
        s.image =
            sprites.frames[(time.elapsed_secs() * SPEED) as usize % sprites.frames.len()].clone();
    }
}

fn load_sprites(mut commands: Commands, asset_loader: Res<AssetServer>) {
    commands.insert_resource(ConveyorSprites {
        frames: vec![
            asset_loader.load("conveyor_0.png"),
            asset_loader.load("conveyor_1.png"),
            asset_loader.load("conveyor_2.png"),
        ],
    });
}

fn snake_3(commands: &mut Commands, asset_server: &AssetServer) {
    // First conveyor is the box spawner.
    create_conveyor_start(commands, asset_server, 1, 3, Vec2::X, None);

    create_line_x(
        commands,
        asset_server,
        2,
        ROOM_CONVEYOR_WIDTH - 3,
        3,
        Vec2::X,
        Vec2::X,
    );

    create_line_y(
        commands,
        asset_server,
        3,
        5,
        ROOM_CONVEYOR_WIDTH - 2,
        Vec2::NEG_Y,
        Vec2::X,
    );

    create_line_x(
        commands,
        asset_server,
        ROOM_CONVEYOR_WIDTH - 2,
        2,
        6,
        Vec2::NEG_X,
        Vec2::NEG_Y,
    );

    create_line_y(commands, asset_server, 6, 8, 1, Vec2::NEG_Y, Vec2::NEG_X);

    create_line_x(
        commands,
        asset_server,
        1,
        ROOM_CONVEYOR_WIDTH - 3,
        9,
        Vec2::X,
        Vec2::NEG_Y,
    );

    // Last conveyor is the box goal.
    create_conveyor_end(
        commands,
        asset_server,
        ROOM_CONVEYOR_WIDTH - 2,
        9,
        Vec2::X,
        None,
    );
}

fn snake_4(commands: &mut Commands, asset_server: &AssetServer) {
    // First conveyor is the box spawner.
    let mut conveyor_commands = create_conveyor(commands, asset_server, 1, 3, Vec2::X, None);
    conveyor_commands.insert(BoxSpawner);

    create_line_x(
        commands,
        asset_server,
        2,
        ROOM_CONVEYOR_WIDTH - 3,
        3,
        Vec2::X,
        Vec2::X,
    );

    create_line_y(
        commands,
        asset_server,
        3,
        5,
        ROOM_CONVEYOR_WIDTH - 2,
        Vec2::NEG_Y,
        Vec2::X,
    );

    create_line_x(
        commands,
        asset_server,
        ROOM_CONVEYOR_WIDTH - 2,
        2,
        6,
        Vec2::NEG_X,
        Vec2::NEG_Y,
    );

    create_line_y(commands, asset_server, 6, 8, 1, Vec2::NEG_Y, Vec2::NEG_X);

    create_line_x(
        commands,
        asset_server,
        1,
        ROOM_CONVEYOR_WIDTH - 3,
        9,
        Vec2::X,
        Vec2::NEG_Y,
    );

    create_line_y(
        commands,
        asset_server,
        9,
        11,
        ROOM_CONVEYOR_WIDTH - 2,
        Vec2::NEG_Y,
        Vec2::X,
    );

    create_line_x(
        commands,
        asset_server,
        ROOM_CONVEYOR_WIDTH - 2,
        2,
        12,
        Vec2::NEG_X,
        Vec2::NEG_Y,
    );

    // Last conveyor is the box goal.
    let mut conveyor_commands = create_conveyor(commands, asset_server, 1, 12, Vec2::NEG_X, None);
    conveyor_commands.insert(BoxGoal);
}

fn line_1(y: u32, commands: &mut Commands, asset_server: &AssetServer) {
    // First conveyor is the box spawner.
    let mut conveyor_commands = create_conveyor(commands, asset_server, 1, y, Vec2::X, None);
    conveyor_commands.insert(BoxSpawner);

    create_line_x(
        commands,
        asset_server,
        2,
        ROOM_CONVEYOR_WIDTH - 3,
        y,
        Vec2::X,
        Vec2::X,
    );

    // Last conveyor is the box goal.
    let mut conveyor_commands = create_conveyor(
        commands,
        asset_server,
        ROOM_CONVEYOR_WIDTH - 2,
        y,
        Vec2::X,
        None,
    );
    conveyor_commands.insert(BoxGoal);
}

fn line_2(commands: &mut Commands, asset_server: &AssetServer) {
    line_1(5, commands, asset_server);
    line_1(11, commands, asset_server);
}

fn spiral(commands: &mut Commands, asset_server: &AssetServer) {
    // First conveyor is the box spawner.
    let mut conveyor_commands = create_conveyor(commands, asset_server, 1, 3, Vec2::X, None);
    conveyor_commands.insert(BoxSpawner);

    create_line_x(
        commands,
        asset_server,
        2,
        ROOM_CONVEYOR_WIDTH - 3,
        3,
        Vec2::X,
        Vec2::X,
    );

    create_line_y(
        commands,
        asset_server,
        3,
        12,
        ROOM_CONVEYOR_WIDTH - 2,
        Vec2::NEG_Y,
        Vec2::X,
    );

    create_line_x(
        commands,
        asset_server,
        ROOM_CONVEYOR_WIDTH - 2,
        2,
        13,
        Vec2::NEG_X,
        Vec2::NEG_Y,
    );

    create_line_y(commands, asset_server, 13, 6, 1, Vec2::Y, Vec2::NEG_X);

    create_line_x(
        commands,
        asset_server,
        1,
        ROOM_CONVEYOR_WIDTH - 5,
        5,
        Vec2::X,
        Vec2::Y,
    );

    create_line_y(
        commands,
        asset_server,
        5,
        10,
        ROOM_CONVEYOR_WIDTH - 4,
        Vec2::NEG_Y,
        Vec2::X,
    );

    create_line_x(
        commands,
        asset_server,
        ROOM_CONVEYOR_WIDTH - 4,
        4,
        11,
        Vec2::NEG_X,
        Vec2::NEG_Y,
    );

    create_line_y(commands, asset_server, 11, 8, 3, Vec2::Y, Vec2::NEG_X);

    create_line_x(
        commands,
        asset_server,
        3,
        ROOM_CONVEYOR_WIDTH - 7,
        7,
        Vec2::X,
        Vec2::Y,
    );

    create_line_y(
        commands,
        asset_server,
        7,
        8,
        ROOM_CONVEYOR_WIDTH - 6,
        Vec2::NEG_Y,
        Vec2::X,
    );

    create_line_x(
        commands,
        asset_server,
        ROOM_CONVEYOR_WIDTH - 6,
        6,
        9,
        Vec2::NEG_X,
        Vec2::NEG_Y,
    );

    // Last conveyor is the box goal.
    let mut conveyor_commands = create_conveyor(commands, asset_server, 5, 9, Vec2::NEG_X, None);
    conveyor_commands.insert(BoxGoal);
}

fn test(commands: &mut Commands, asset_server: &AssetServer) {
    // First conveyor is the box spawner.
    let mut conveyor_commands = create_conveyor(commands, asset_server, 5, 5, Vec2::Y, None);
    conveyor_commands.insert(BoxSpawner);

    create_conveyor(commands, asset_server, 5, 4, Vec2::Y, Some(Vec2::X));
    create_conveyor(commands, asset_server, 6, 4, Vec2::X, None);
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, (setup_room, load_sprites));
    app.add_systems(
        Update,
        (animate_conveyors, move_thing_on_conveyor).run_if(in_state(GameState::Running)),
    );
}
