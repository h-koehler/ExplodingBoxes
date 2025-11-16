use bevy::{color::palettes::css, prelude::*};

use crate::levels::Level;

pub mod button;
pub mod loss;
pub mod win;

pub const UI_HEIGHT: f32 = 200.0;
pub const LEVEL: &str = "LEVEL";

pub enum BadAttributes {
    Color(Srgba),
    Symbol(String),
}

#[derive(Resource)]
pub struct UIBad {
    pub bad_attributes: Vec<BadAttributes>,
    pub additional_text: Vec<String>,
}

fn create_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    bad_box: Res<UIBad>,
    level: Res<Level>,
) {
    let level_num = (*level as i16) + 1;
    commands
        .spawn((Node {
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(100.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Row,
            padding: UiRect::all(Val::Px(10.0)),
            ..Default::default()
        },))
        .with_children(|p| {
            p.spawn((
                Name::new("Level Text"),
                Node {
                    margin: UiRect::horizontal(Val::Px(5.0)),
                    ..Default::default()
                },
            ))
            .with_child((
                Text::new(LEVEL),
                TextFont {
                    font: asset_server.load("fonts/default.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));
        })
        .with_children(|p| {
            p.spawn((
                Name::new("Level Number Text"),
                Node {
                    margin: UiRect::horizontal(Val::Px(5.0)),
                    ..Default::default()
                },
            ))
            .with_child((
                Text::new(level_num.to_string()),
                TextFont {
                    font: asset_server.load("fonts/default.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));
        });

    commands
        .spawn((
            Node {
                bottom: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Px(UI_HEIGHT),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(20.0)),
                ..Default::default()
            },
            ImageNode::new(asset_server.load("ui_elements/ui_background.png")),
        ))
        .with_children(|p| {
            p.spawn((
                Node {
                    margin: UiRect::axes(Val::Px(5.0), Val::Auto),
                    width: Val::Px(128.0),
                    height: Val::Px(128.0),
                    ..Default::default()
                },
                ImageNode::new(asset_server.load("ui_elements/X.png")),
            ));

            for bad_item in bad_box.bad_attributes.iter() {
                match bad_item {
                    BadAttributes::Symbol(symbol) => {
                        p.spawn((
                            Node {
                                margin: UiRect::axes(Val::Px(5.0), Val::Auto),
                                width: Val::Px(128.0),
                                height: Val::Px(128.0),
                                ..Default::default()
                            },
                            ImageNode::new(asset_server.load(format!("addons/{symbol}.png"))),
                        ));
                    }
                    BadAttributes::Color(color) => {
                        p.spawn((
                            Name::new("Symbol"),
                            Node {
                                margin: UiRect::axes(Val::Px(5.0), Val::Auto),
                                width: Val::Px(128.0),
                                height: Val::Px(128.0),
                                ..Default::default()
                            },
                            ImageNode::new(asset_server.load(format!("neutral/simple.png")))
                                .with_color((*color).into()),
                        ));
                    }
                }
            }

            p.spawn(
                (Node {
                    flex_grow: 1.0,
                    ..Default::default()
                }),
            );

            p.spawn(
                (Node {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                }),
            )
            .with_children(|p| {
                if !bad_box.additional_text.is_empty() {
                    p.spawn((
                        Name::new("TEXT!"),
                        Text::new("NEW INSTRUCTIONS"),
                        TextFont {
                            font_size: 24.0,
                            font: asset_server.load("fonts/default.ttf"),
                            ..Default::default()
                        },
                        TextColor(css::RED.into()),
                        Node {
                            margin: UiRect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                    ));
                }
                for text in bad_box.additional_text.iter() {
                    p.spawn((
                        Name::new("TEXT!"),
                        Text::new(text),
                        TextFont {
                            font_size: 24.0,
                            // font: asset_server.load("fonts/default.TTF"),
                            ..Default::default()
                        },
                        Node {
                            margin: UiRect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                    ));
                }
            });
        });
}

pub(super) fn register(app: &mut App) {
    button::register(app);
    loss::register(app);
    win::register(app);
    app.add_systems(
        Update,
        create_ui.run_if(resource_exists_and_changed::<UIBad>),
    );
}
