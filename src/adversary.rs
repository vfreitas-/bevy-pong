use bevy::prelude::*;
use crate::paddle::spawn_paddle;

pub struct AdversaryPlugin;

impl Plugin for AdversaryPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(adversary_setup);
  }
}

#[derive(Component)]
pub struct Adversary;

fn adversary_setup (
  commands: Commands,
) {
  spawn_paddle(commands, Vec2::new(-25., 0.), Adversary);
}