use bevy::{color::palettes::css, prelude::*};

use crate::{
    levels::level_select::{self, show_select_screen},
    ui::button::{ButtonMessage, ButtonStyles, CosmosButton},
};

pub const LEVEL_SELECT: &str = "LEVEL SELECT";

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
    BossCatTime,
}

#[derive(Component)]
pub struct PauseMenu;

fn toggle_pause(
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    q_pause: Query<Entity, With<PauseMenu>>,
    asset_server: Res<AssetServer>,
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::Running => {
                next_state.set(GameState::Paused);
                commands
                    .spawn((
                        PauseMenu,
                        Node {
                            margin: UiRect::all(Val::Auto),
                            width: Val::Px(600.0),
                            height: Val::Px(600.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::End,
                            ..Default::default()
                        },
                        ImageNode::new(asset_server.load("ui_elements/pause_menu.png")),
                    ))
                    .with_children(|p| {
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
                                    LEVEL_SELECT.into(),
                                    TextFont {
                                        font: asset_server.load("fonts/ARCADECLASSIC.ttf"),
                                        font_size: 33.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                )),
                                ..Default::default()
                            },
                            Node {
                                border: UiRect::all(Val::Px(3.0)),
                                margin: UiRect::all(Val::Percent(10.0)),
                                width: Val::Auto,
                                height: Val::Px(30.0),
                                padding: UiRect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            BorderColor::all(css::BLACK),
                        ))
                        .observe(
                            |trigger: On<ButtonMessage>,
                             commands: Commands,
                             asset_server: Res<AssetServer>| {
                                show_select_screen(commands, asset_server);
                            },
                        );
                    });
            }
            GameState::Paused => {
                next_state.set(GameState::Running);
                if let Ok(pause_ent) = q_pause.single() {
                    commands.entity(pause_ent).despawn();
                }
            }
            GameState::BossCatTime => next_state.set(GameState::Paused),
        }
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Update, toggle_pause);
    app.init_state::<GameState>();
}
