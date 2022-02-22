use bevy::prelude::*;
use heron::prelude::*;
use crate::physics::Layer;

#[derive(Component)]
pub struct Paddle;

pub fn spawn_paddle (
  mut commands: Commands,
  position: Vec2,
  component: impl Component,
) {
  commands.spawn_bundle(
    SpriteBundle {
      sprite: Sprite {
        color: Color::hex("ffffff").unwrap(),
        ..Default::default()
      },
      transform: Transform {
        translation: Vec3::new(position.x, position.y, 1.0),
        scale: Vec3::new(1.0, 5.0, 1.0),
        ..Default::default()
      },
      ..Default::default()
    }
  )
  .insert(RigidBody::KinematicVelocityBased)
  .insert(CollisionShape::Cuboid { 
    half_extends: Vec3::new(0.5, 2.5, 1.0),
    border_radius: None
  })
  // .insert(Velocity::from_linear(Vec3::X * 2.0))
  // .insert(Acceleration::from_linear(Vec3::X * 1.0))
  .insert(PhysicMaterial { 
    friction: 1.0,
    density: 9999.0,
    restitution: 1.0,
  })
  .insert(RotationConstraints::lock())
  .insert(CollisionLayers::none()
    .with_group(Layer::Paddle)
    .with_masks(&[Layer::World, Layer::Ball]))
  .insert(Paddle)
  .insert(component);
}
