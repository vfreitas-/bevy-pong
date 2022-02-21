use bevy::prelude::*;
use crate::paddle::spawn_paddle;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(player_setup);
  }
}

#[derive(Component)]
pub struct Player;

fn player_setup (
  commands: Commands,
) {
  spawn_paddle(commands, Vec2::new(26., 0.), Player);
}