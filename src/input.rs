use bevy::prelude::*;
use crate::GameState;

pub struct InputPlugin;

impl Plugin for InputPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(input);
  }
}

fn input (
  mut state: ResMut<State<GameState>>,
  keyboard_input: Res<Input<KeyCode>>,
) {
  if keyboard_input.just_released(KeyCode::Return) {
    if &GameState::Start == state.current() {
      state.set(GameState::Playing).unwrap()
    } else if &GameState::Playing == state.current() {
      state.set(GameState::Start).unwrap();
    }
  }
}