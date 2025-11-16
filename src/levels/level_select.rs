use bevy::{color::palettes::css, prelude::*};

use crate::{
    levels::{Level, setup_level},
    ui::button::{ButtonMessage, ButtonStyles, CosmosButton},
};

pub const LEVEL_SELECT: &str = "LEVEL SELECT";
pub const ALL_LEVELS: [Level; 5] = [
    Level::One,
    Level::Two,
    Level::Three,
    Level::Four,
    Level::Five,
];

#[derive(Component)]
pub struct LevelSelectMenu;

pub fn show_select_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("Level Select Background"),
            LevelSelectMenu,
            Node {
                width: Val::Px(1100.0),
                height: Val::Px(700.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_items: JustifyItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                ..Default::default()
            },
            ImageNode::new(asset_server.load("ui_elements/level_select_bg.png")),
        ))
        .with_children(|p| {
            p.spawn((
                Name::new("Level Select Title"),
                Node {
                    margin: UiRect::axes(Val::Px(5.0), Val::Px(50.0)),
                    height: Val::Px(100.0),
                    ..Default::default()
                },
            ))
            .with_child((
                Text::new(LEVEL_SELECT),
                TextFont {
                    font: asset_server.load("fonts/ARCADECLASSIC.ttf"),
                    font_size: 48.0,
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
                for level_enum in ALL_LEVELS.iter().copied() {
                    let level_num = match level_enum {
                        Level::One => "1",
                        Level::Two => "2",
                        Level::Three => "3",
                        Level::Four => "4",
                        Level::Five => "5",
                    };
                    p.spawn((
                        CosmosButton {
                            button_styles: Some(ButtonStyles {
                                background_color: Srgba::rgba_u8(0, 0, 0, 0).into(),
                                hover_background_color: Srgba::rgba_u8(50, 50, 50, 255).into(),
                                press_background_color: css::BLACK.into(),
                                press_foreground_color: css::WHITE.into(),
                                ..Default::default()
                            }),
                            text: Some((
                                level_num.into(),
                                TextFont {
                                    font: asset_server.load("fonts/ARCADECLASSIC.ttf"),
                                    font_size: 48.0,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                            )),
                            ..Default::default()
                        },
                        Name::new(format!("Level {level_num}")),
                        Node {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..Default::default()
                        },
                        ImageNode::new(asset_server.load("ui_elements/level_select_bg")),
                    ))
                    .observe(
                        move |trigger: On<ButtonMessage>, mut commands: Commands, mut level: ResMut<Level>, q_level_select_menu: Query<Entity, With<LevelSelectMenu>>| {
                            *level = level_enum;
                            
                            if let Ok(menu_ent) = q_level_select_menu.single() {
                              commands.entity(menu_ent).despawn();
                            } 

                            commands.insert_resource(JustReset)
                        },
                    );
                }
            });
        });
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Update, show_select_screen);
}
