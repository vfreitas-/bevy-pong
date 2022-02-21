use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod physics;
mod camera;
mod ball;
mod paddle;
mod player;
mod adversary;

use physics::AppPhysicsPlugin;
use player::PlayerPlugin;
use adversary::AdversaryPlugin;
use camera::CameraPlugin;
use ball::BallPlugin;

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
    .add_plugin(WorldInspectorPlugin::new())
    .add_plugin(AppPhysicsPlugin)
    .add_plugin(CameraPlugin)
    .add_plugin(BallPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(AdversaryPlugin)
    .run();
}