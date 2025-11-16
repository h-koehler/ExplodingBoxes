use bevy::{color::palettes::css, prelude::*};

use crate::{
    boxes::GameBox,
    character_controls::{Character, Velocity, swat::DidBadSwat},
    custom_utils::GameState,
    levels::{Level, advance::JustReset},
    ui::button::{ButtonMessage, CosmosButton},
};

#[derive(Message, Default)]
pub struct Win;

#[derive(Component)]
struct WinMenu;

fn show_win(
    mut mr_loss: MessageReader<Win>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
) {
    let Some(_) = mr_loss.read().next() else {
        return;
    };

    next_state.set(GameState::Win);

    let font = asset_server.load("fonts/default.ttf");

    commands
        .spawn((
            WinMenu,
            Name::new("Win menu"),
            Node {
                margin: UiRect::all(Val::Auto),
                width: Val::Px(600.0),
                height: Val::Px(400.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BackgroundColor(
                Srgba {
                    red: 0.3,
                    green: 0.3,
                    blue: 0.3,
                    alpha: 0.9,
                }
                .into(),
            ),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("YOU WON!"),
                TextFont {
                    font: font.clone(),
                    font_size: 50.0,
                    ..Default::default()
                },
                TextColor(css::GREEN.into()),
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..Default::default()
                },
            ));

            p.spawn((
                CosmosButton {
                    text: Some((
                        "Play Again?".into(),
                        TextFont {
                            font: font.clone(),
                            font_size: 30.0,
                            ..Default::default()
                        },
                        Default::default(),
                    )),
                    ..Default::default()
                },
                Node {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..Default::default()
                },
            ))
            .observe(
                |_: On<ButtonMessage>,
                 q_boxes: Query<Entity, With<GameBox>>,
                 mut level: ResMut<Level>,
                 mut commands: Commands,
                 q_loss_menu: Query<Entity, With<WinMenu>>,
                 q_character: Query<Entity, With<Character>>,
                 mut state: ResMut<NextState<GameState>>| {
                    for b in q_boxes.iter() {
                        commands.entity(b).despawn();
                    }

                    *level = Level::One;
                    commands
                        .entity(q_character.single().unwrap())
                        .insert((
                            Transform::from_translation(Vec3::Z * 3.0),
                            Velocity::default(),
                        ))
                        .remove::<DidBadSwat>();

                    for e in q_loss_menu.iter() {
                        commands.entity(e).despawn();
                    }

                    commands.insert_resource(JustReset);

                    state.set(GameState::Running);
                },
            );
        });
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Update, show_win).add_message::<Win>();
}
