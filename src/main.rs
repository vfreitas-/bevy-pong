use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod physics;
mod camera;
mod ball;
mod paddle;
mod player;
mod adversary;
mod level;
mod ui;

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
    .add_plugin(physics::AppPhysicsPlugin)
    .add_plugin(camera::CameraPlugin)
    .add_plugin(ui::UIPlugin)
    .add_plugin(level::LevelPlugin)
    .add_plugin(ball::BallPlugin)
    .add_plugin(player::PlayerPlugin)
    .add_plugin(adversary::AdversaryPlugin)
    .add_system(bevy::input::system::exit_on_esc_system)
    .run();
}