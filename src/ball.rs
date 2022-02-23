use bevy::prelude::*;
use heron::prelude::*;
use rand::Rng;
use crate::BALL_SPEED;
use crate::level::GoalLine;
use crate::level::GoalLineSide;
use crate::physics::Layer;
use crate::GameState;
use crate::score::*;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum BallLabel {
  Movement,
  Collision,
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(setup)
      .add_system_set(
        SystemSet::on_enter(GameState::Playing)
          .with_system(ball_initialize)
      )
      .add_system_set(
        SystemSet::on_update(GameState::Playing)
          .with_system(
            ball_detect_collisions
              .label(BallLabel::Collision)
            )
          .with_system(
            ball_movement
              .label(BallLabel::Movement)
              .after(BallLabel::Collision)
          )
      );
  }
}

#[derive(Component)]
struct Ball {
  target: Vec3,
}

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
  .insert(Ball { target: Vec3::ZERO });
}

fn ball_initialize (
  mut q: Query<(&mut Transform, &mut Ball)>
) {
  for (
    mut transform,
    mut ball
  ) in q.iter_mut() {
    let rand_x = rand::thread_rng().gen_range(-1., 1.);
    let rand_y = rand::thread_rng().gen_range(-1., 1.);
    let direction = Vec3::new(rand_x, rand_y, 1.);
    ball.target = direction.normalize() * 100.;
    transform.translation = Vec3::new(0., 0., 1.);
  }
}

fn ball_movement (
  time: Res<Time>,
  mut q: Query<(&mut Velocity, &Transform, &Ball)>
) {
  for (
    mut velocity,
    transform,
    ball
  ) in q.iter_mut() {
    let direction = Vec3::new(
      ball.target.x - transform.translation.x, 
      ball.target.y - transform.translation.y, 1.
    ).normalize();

    velocity.linear = direction * BALL_SPEED * time.delta_seconds();
  }
}

fn ball_detect_collisions(
  mut q_ball: Query<(Entity, &mut Ball, &Velocity)>,
  q_goal_line: Query<(Entity, &mut GoalLine)>,
  mut events: EventReader<CollisionEvent>,
  mut on_score_writer: EventWriter<OnScore>,

) {
  for event in events.iter() {
    if let CollisionEvent::Started(data1, data2) = event {
      for (entity, mut ball, velocity) in q_ball.iter_mut() {
        if entity == data1.rigid_body_entity() || entity == data2.rigid_body_entity() {
          for (goalline_entity, goalline) in q_goal_line.iter() {
            if data1.rigid_body_entity() == goalline_entity || data2.rigid_body_entity() == goalline_entity {
              if GoalLineSide::Left == goalline.side {
                on_score_writer.send(OnScore(GoalLineSide::Left));
              } else if GoalLineSide::Right == goalline.side {
                on_score_writer.send(OnScore(GoalLineSide::Right));
              }
              return;
            }
          }
          ball.target = get_bounce(velocity.linear, *data2.normals().first().unwrap()) * 100.;
          // velocity.linear = get_bounce(velocity.linear, *data2.normals().first().unwrap());
          return;
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
