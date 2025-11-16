use bevy::prelude::*;

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
                            Node {
                                margin: UiRect::all(Val::Percent(10.0)),
                                width: Val::Auto,
                                height: Val::Px(20.0),
                                ..Default::default()
                            },
                            ImageNode::new(asset_server.load("ui_elements/level_select.png")),
                        ));
                        p.spawn((
                            Node {
                                margin: UiRect::all(Val::Percent(10.0)),
                                width: Val::Auto,
                                height: Val::Px(20.0),
                                ..Default::default()
                            },
                            ImageNode::new(asset_server.load("ui_elements/retry.png")),
                        ));
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
