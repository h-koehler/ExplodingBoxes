use bevy::prelude::*;

use crate::levels::Level;

pub const LEVEL_SELECT: &str = "LEVEL SELECT";
pub const ALL_LEVELS: [Level; 5] = [
    Level::One,
    Level::Two,
    Level::Three,
    Level::Four,
    Level::Five,
];

fn level_select(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("Level Select Background"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..Default::default()
            },
            ImageNode::new(asset_server.load("ui_elements/level_select_bg.png")),
        ))
        .with_children(|p| {
            p.spawn((
                Name::new("Level Select Title"),
                Node {
                    margin: UiRect::horizontal(Val::Px(5.0)),
                    ..Default::default()
                },
            ))
            .with_child((
                Text::new(LEVEL_SELECT),
                TextFont {
                    font: asset_server.load("fonts/ARCADECLASSIC.ttf"),
                    font_size: 240.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));

            p.spawn((
                Name::new("Level Selection"),
                Node {
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
            ))
            .with_children(|p| {
                for level in ALL_LEVELS.iter() {
                    let level_num = match level {
                        Level::One => "1",
                        Level::Two => "2",
                        Level::Three => "3",
                        Level::Four => "4",
                        Level::Five => "5",
                    };
                    p.spawn((
                        Name::new(format!("Level {level_num}")),
                        Node {
                            margin: UiRect::horizontal(Val::Px(5.0)),
                            ..Default::default()
                        },
                        ImageNode::new(asset_server.load("ui_elements/level_select_bg")),
                    ))
                    .with_child((
                        Text::new(level_num),
                        TextFont {
                            font: asset_server.load("fonts/ARCADECLASSIC.ttf"),
                            font_size: 240.0,
                            ..Default::default()
                        },
                        TextColor(Color::BLACK),
                    ));
                }
            });
        });
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Update, level_select);
}
