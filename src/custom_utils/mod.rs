use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
    BossCatTime,
}

fn toggle_pause(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::Running => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Running),
            GameState::BossCatTime => next_state.set(GameState::Paused),
        }
    }
}

pub(super) fn register(app: &mut App) {
        app.add_systems(Update, toggle_pause);
        app.init_state::<GameState>();
}
