use bevy::prelude::*;

use crate::levels::Level;

pub mod button;

pub const UI_HEIGHT: f32 = 200.0;

pub enum BadAttributes {
    Color(Srgba),
    Symbol(String),
}

#[derive(Resource)]
pub struct UIBad {
    pub bad_attributes: Vec<BadAttributes>,
}

#[derive(Component)]
pub struct Button;

#[derive(Component)]
pub struct UILevel;

#[derive(Component)]
pub struct UIBadItem;

fn create_ui(
    mut commands: Commands,
    q_level: Query<Entity, With<UILevel>>,
    q_bad_item: Query<Entity, With<UIBadItem>>,
    asset_server: Res<AssetServer>,
    bad_box_rules: Res<UIBad>,
    level: Res<Level>,
) {
    if let Ok(level_ent) = q_level.single() {
        commands.entity(level_ent).despawn();
    }

    for bad_item in q_bad_item.iter() {
        commands.entity(bad_item).despawn();
    }

    commands
        .spawn((Node {
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(100.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Row,
            padding: UiRect::all(Val::Px(20.0)),
            ..Default::default()
        },))
        .with_children(|p| {
            p.spawn((
                Node {
                    margin: UiRect::horizontal(Val::Px(5.0)),
                    width: Val::Px(100.0),
                    height: Val::Px(20.0),
                    ..Default::default()
                },
                ImageNode::new(asset_server.load("ui_elements/level.png")),
            ));

            let level_num = (*level as i16) + 1;
            p.spawn((
                UILevel,
                Node {
                    margin: UiRect::horizontal(Val::Px(5.0)),
                    width: Val::Px(20.0),
                    height: Val::Px(20.0),
                    ..Default::default()
                },
                ImageNode::new(asset_server.load(format!("ui_elements/{level_num}.png"))),
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

            for bad_item in bad_box_rules.bad_attributes.iter() {
                match bad_item {
                    BadAttributes::Symbol(symbol) => {
                        p.spawn((
                            UIBadItem,
                            Node {
                                margin: UiRect::axes(Val::Px(5.0), Val::Auto),
                                width: Val::Px(128.0),
                                height: Val::Px(128.0),
                                ..Default::default()
                            },
                            ImageNode::new(asset_server.load(format!("bad/{symbol}.png"))),
                        ));
                    }
                    BadAttributes::Color(color) => {
                        p.spawn((
                            UIBadItem,
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
        });
}

pub(super) fn register(app: &mut App) {
    button::register(app);

    app.add_systems(
        Update,
        create_ui.run_if(resource_exists_and_changed::<UIBad>),
    );
}
