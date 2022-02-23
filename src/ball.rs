use bevy::prelude::*;
use heron::prelude::*;
use crate::level::GoalLine;
use crate::level::GoalLineSide;
use crate::physics::Layer;
use crate::GameState;
use crate::score::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(setup)
      .add_system_set(
        SystemSet::on_enter(GameState::Playing)
          .with_system(ball_impulse)
      )
      .add_system_set(
        SystemSet::on_update(GameState::Playing)
          .with_system(ball_detect_collisions)
      );
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
  .insert(Velocity::from_linear(Vec3::ZERO))
  .insert(Acceleration::from_linear(Vec3::ZERO))
  .insert(PhysicMaterial { friction: 1.0, density: 10.0, ..Default::default() })
  .insert(RotationConstraints::lock())
  .insert(CollisionLayers::none()
    .with_group(Layer::Ball)
    .with_masks(&[Layer::World, Layer::Paddle, Layer::GoalLine])
  )
  .insert(Ball);
}

fn ball_impulse (
  mut q: Query<(&mut Velocity, &mut Transform), With<Ball>>
) {
  for (mut velocity, mut transform) in q.iter_mut() {
    let rand = rand::random::<f32>();
    transform.translation = Vec3::new(0., 0., 1.);
    velocity.linear = Vec3::X * 15.;
    // velocity.linear = Vec3::new(rand * 20., rand * 20., 1.);
  }
}

fn ball_detect_collisions(
  time: Res<Time>,
  mut commands: Commands,
  mut qBall: Query<(Entity, &mut Velocity), With<Ball>>,
  mut qGoalLine: Query<(Entity, &mut GoalLine)>,
  mut events: EventReader<CollisionEvent>,
  mut on_score_writer: EventWriter<OnScore>,

) {
  for event in events.iter() {
    if let CollisionEvent::Started(data1, data2) = event {
      let layers1 = data1.collision_layers();
      let layers2 = data2.collision_layers();

      for (ball, mut velocity) in qBall.iter_mut() {
        if ball == data1.rigid_body_entity() || ball == data2.rigid_body_entity() {
          for (goalline_entity, goalline) in qGoalLine.iter() {
            if data1.rigid_body_entity() == goalline_entity || data2.rigid_body_entity() == goalline_entity {
              if GoalLineSide::Left == goalline.side {
                on_score_writer.send(OnScore(GoalLineSide::Left));
              } else if GoalLineSide::Right == goalline.side {
                on_score_writer.send(OnScore(GoalLineSide::Right));
              }
              return;
            }
          }

          velocity.linear = get_bounce(velocity.linear, *data2.normals().first().unwrap());
        }
      }
    };
  }
}

fn get_bounce (
  current_velocity: Vec3,
  normal: Vec2,
) -> Vec3 {
  let normal = normal.normalize();
  let velocity = Vec2::new(current_velocity.x, current_velocity.y);
  let dot = velocity.dot(normal);
  let reflect = 2. * normal * dot - velocity;
  let bounce = -reflect;
  return Vec3::new(bounce.x, bounce.y, 1.);
}
