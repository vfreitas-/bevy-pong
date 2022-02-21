use bevy::prelude::*;
use heron::prelude::*;
use crate::physics::Layer;

pub struct BallPlugin;

impl Plugin for BallPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(setup);
  }
}

#[derive(Component)]
struct Ball;

fn setup (mut commands: Commands) {
  commands.spawn_bundle(
    SpriteBundle {
      sprite: Sprite {
        color: Color::hex("ffffff").unwrap(),
        ..Default::default()
      },
      ..Default::default()
    }
  )
  .insert(RigidBody::Dynamic)
  .insert(CollisionShape::Sphere { radius: 0.75 })
  .insert(Velocity::from_linear(Vec3::X * 10.0))
  .insert(Acceleration::from_linear(Vec3::X * 5.0))
  .insert(PhysicMaterial { friction: 1.0, density: 10.0, ..Default::default() })
  .insert(RotationConstraints::lock())
  .insert(CollisionLayers::none()
    .with_group(Layer::Ball)
    .with_mask(Layer::World)
    .with_mask(Layer::Paddle))
  .insert(Ball);
}

fn ball_movement (
  mut commands: Commands,
  mut q: Query<(&mut Ball, &mut Transform)>
) {
  for (ball, transform) in q.iter_mut() {
    
  }
}
