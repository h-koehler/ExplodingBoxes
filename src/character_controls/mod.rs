use bevy::prelude::*;

#[derive(Component)]
pub struct Character;

fn move_smile(
    inputs: Res<ButtonInput<KeyCode>>,
    mut q_camera_transform: Query<&mut Transform, With<Character>>,
) {
    let mut char_trans = q_camera_transform.single_mut().expect("No char trams ;(");

    if inputs.pressed(KeyCode::KeyW) {
        char_trans.translation.y += 1.0;
    }
    if inputs.pressed(KeyCode::KeyS) {
        char_trans.translation.y -= 1.0;
    }
    if inputs.pressed(KeyCode::KeyA) {
        char_trans.translation.x -= 1.0;
    }
    if inputs.pressed(KeyCode::KeyD) {
        char_trans.translation.x += 1.0;
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d::default(),));

    commands.spawn((
        Character,
        Sprite::from_image(asset_server.load("smile.png")),
    ));
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, move_smile);
}
