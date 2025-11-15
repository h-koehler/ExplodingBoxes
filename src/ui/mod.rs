use bevy::{color::palettes::css, prelude::*};

pub const UI_HEIGHT: f32 = 200.0;

fn create_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            bottom: Val::Px(0.0), // bottom: 0px
            width: Val::Percent(100.0),
            height: Val::Px(UI_HEIGHT),
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        BackgroundColor(css::WHITE.into()),
    ));
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Startup, create_ui);
}
