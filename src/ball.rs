use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
  .insert_bundle(
    RigidBodyBundle {
      position: Vec2::new(0.0, 0.0).into(),
      velocity: RigidBodyVelocity {
        linvel: Vec2::new(1.0, 2.0).into(),
        angvel: 0.4
      }.into(),
      forces: RigidBodyForces {
        force: Vec2::new(1000.0, 2000.0).into(),
        gravity_scale: 2.0,
        ..Default::default()
      }.into(),
      ..Default::default()
    }
  )
  .insert_bundle(
    ColliderBundle {
      shape: ColliderShape::ball(1.).into(),
      ..Default::default()
    }
  )
  .insert(RigidBodyPositionSync::Discrete)
  .insert(Ball);
}

fn ball_movement (
  mut commands: Commands,
  mut q: Query<(&mut Ball, &mut Transform)>
) {
  for (ball, transform) in q.iter_mut() {
    
  }
}