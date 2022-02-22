use bevy::prelude::*;
use heron::prelude::*;
use crate::physics::Layer;

pub struct BallPlugin;

impl Plugin for BallPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(setup)
      .add_system(detect_collisions);
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
  .insert(Acceleration::from_linear(Vec3::new(1., 1., 1.)))
  .insert(PhysicMaterial { friction: 1.0, density: 10.0, ..Default::default() })
  .insert(RotationConstraints::lock())
  .insert(CollisionLayers::none()
    .with_group(Layer::Ball)
    .with_masks(&[Layer::World, Layer::Paddle])
  )
  .insert(Ball);
}

fn ball_movement (
  mut commands: Commands,
  mut q: Query<(&mut Ball, &mut Transform)>
) {
  for (ball, transform) in q.iter_mut() {
    
  }
}

fn detect_collisions(
  time: Res<Time>,
  mut q: Query<(Entity, &mut Velocity), With<Ball>>,
  mut events: EventReader<CollisionEvent>
) {
  for event in events.iter() {
    if let CollisionEvent::Started(data1, data2) = event {
      // println!("event: {:?}", data1);
      for (ball, mut velocity) in q.iter_mut() {
        if ball == data1.rigid_body_entity() || ball == data2.rigid_body_entity() {
          let normal = data2.normals().first().unwrap();
          println!("velocity: {:?}", velocity.linear);
          velocity.linear = Vec3::new(normal.x, normal.y, 1.) * 1000. * time.delta_seconds();
        }
      }
    };
  }
}
