use bevy::prelude::*;
use heron::prelude::*;

#[derive(PhysicsLayer)]
pub enum Layer {
  World,
  Ball,
  Paddle,
}

pub struct AppPhysicsPlugin;

impl Plugin for AppPhysicsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(PhysicsPlugin::default())
      .insert_resource(Gravity::from(Vec3::new(0.0, 0.0, 0.0)));
  }
}




