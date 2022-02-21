mod camera;
mod ball;
mod paddle;
mod player;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use paddle::*;
use player::*;
use camera::*;
use ball::BallPlugin;

fn setup (
  commands: Commands,
  q: Query<&mut OrthographicProjection>,
) {
  spawn_paddle(commands, Vec2::new(-26., 0.), Player);
}

fn main() {
  App::new()
    .insert_resource(
      WindowDescriptor {
        title: "Pong!".to_string(),
        width: 640.0,
        height: 480.0,
        ..Default::default()
      }
    )
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(WorldInspectorPlugin::new())
    .add_plugin(BallPlugin)
    .add_startup_system(setup_camera)
    .add_startup_system(setup)
    .run();
}