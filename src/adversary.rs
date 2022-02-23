use bevy::prelude::*;
use heron::Velocity;
use crate::{paddle::*, GameState, PADDLE_SPEED};

pub struct AdversaryPlugin;

impl Plugin for AdversaryPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(adversary_setup)
      .add_system_set(
        SystemSet::on_update(GameState::Playing)
          .with_system(adversary_movement)
      );
  }
}

#[derive(Component)]
pub struct Adversary;

fn adversary_setup (
  commands: Commands,
) {
  spawn_paddle(commands, Vec2::new(-25., 0.), Adversary);
}

fn adversary_movement (
  time: Res<Time>,
  keyboard_input: Res<Input<KeyCode>>,
  mut query: Query<&mut Velocity, With<Adversary>>,
) {
  if let Some(mut velocity) = query.iter_mut().next() {
    let dir = if keyboard_input.pressed(KeyCode::W) {
      1.
    } else if keyboard_input.pressed(KeyCode::S) {
      -1.
    } else {
      0.
    };

    velocity.linear = Vec3::Y * dir * PADDLE_SPEED * time.delta_seconds();
  }
}
