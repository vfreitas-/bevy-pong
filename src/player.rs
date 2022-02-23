use bevy::prelude::*;
use heron::Velocity;
use crate::PADDLE_SPEED;
use crate::paddle::*;
use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(player_setup)
      .add_system_set(
        SystemSet::on_update(GameState::Playing)
          .with_system(player_movement)
      );
  }
}

#[derive(Component)]
pub struct Player;

fn player_setup (
  commands: Commands,
) {
  spawn_paddle(commands, Vec2::new(25., 0.), Player);
}

fn player_movement (
  time: Res<Time>,
  keyboard_input: Res<Input<KeyCode>>,
  mut query: Query<&mut Velocity, With<Player>>,
) {
  if let Some(mut velocity) = query.iter_mut().next() {
    let dir = if keyboard_input.pressed(KeyCode::Up) {
      1.
    } else if keyboard_input.pressed(KeyCode::Down) {
      -1.
    } else {
      0.
    };

    velocity.linear = Vec3::Y * dir * PADDLE_SPEED * time.delta_seconds();
  }
}