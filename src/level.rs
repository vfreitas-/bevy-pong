use bevy::prelude::*;
use heron::prelude::*;

use crate::physics::Layer;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(level_setup);
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GoalLineSide {
  Left,
  Right,
}

#[derive(Component)]
struct Block;

#[derive(Component)]
pub struct GoalLine {
  pub side: GoalLineSide,
}

fn level_setup (
  mut commands: Commands,
) {
  commands
    .spawn_bundle(
      SpriteBundle {
        sprite: Sprite {
          color: Color::hex("ffffff").unwrap(),
          ..Default::default()
        },
        transform: Transform {
          translation: Vec3::new(0., 19.5, 1.0),
          scale: Vec3::new(53.0, 1.0, 1.0),
          ..Default::default()
        },
        ..Default::default()
      }
    )
    .insert(RigidBody::Static)
    .insert(CollisionShape::Cuboid { 
      half_extends: Vec3::new(26.5, 0.5, 1.0),
      border_radius: None
    })
    .insert(PhysicMaterial { 
      friction: 0.0,
      density: 9999.0,
      restitution: 1.0,
    })
    .insert(CollisionLayers::none()
      .with_group(Layer::World)
      .with_masks(&[Layer::Ball, Layer::Paddle])
    )
    .insert(Block);

  commands
    .spawn_bundle(
      SpriteBundle {
        sprite: Sprite {
          color: Color::hex("ffffff").unwrap(),
          ..Default::default()
        },
        transform: Transform {
          translation: Vec3::new(0., -19.5, 1.0),
          scale: Vec3::new(53.0, 1.0, 1.0),
          ..Default::default()
        },
        ..Default::default()
      }
    )
    .insert(RigidBody::Static)
    .insert(CollisionShape::Cuboid { 
      half_extends: Vec3::new(26.5, 0.5, 1.0),
      border_radius: None
    })
    .insert(PhysicMaterial { 
      friction: 0.0,
      density: 9999.0,
      restitution: 1.0,
    })
    .insert(RotationConstraints::lock())
    .insert(CollisionLayers::none()
      .with_group(Layer::World)
      .with_masks(&[Layer::Ball, Layer::Paddle])
    )
    .insert(Block);

  commands.spawn_bundle(
    SpriteBundle {
      sprite: Sprite {
        color: Color::hex("ffffff66").unwrap(),
        ..Default::default()
      },
      transform: Transform {
        translation: Vec3::new(-26.5, 0., 1.0),
        scale: Vec3::new(0.5, 40., 1.),
        ..Default::default()
      },
      ..Default::default()
    },
  )
  .insert(RigidBody::Sensor)
  .insert(CollisionShape::Cuboid { 
    half_extends: Vec3::new(0.25, 20., 1.),
    border_radius: None
  })
  .insert(CollisionLayers::none()
    .with_group(Layer::GoalLine)
    .with_masks(&[Layer::Ball])
  )
  .insert(GoalLine { side: GoalLineSide::Left });

  commands.spawn_bundle(
    SpriteBundle {
      sprite: Sprite {
        color: Color::hex("ffffff66").unwrap(),
        ..Default::default()
      },
      transform: Transform {
        translation: Vec3::new(26.5, 0., 1.0),
        scale: Vec3::new(0.5, 40., 1.),
        ..Default::default()
      },
      ..Default::default()
    },
  )
  .insert(RigidBody::Sensor)
  .insert(CollisionShape::Cuboid { 
    half_extends: Vec3::new(0.25, 20., 1.),
    border_radius: None
  })
  .insert(CollisionLayers::none()
    .with_group(Layer::GoalLine)
    .with_masks(&[Layer::Ball])
  )
  .insert(GoalLine { side: GoalLineSide::Right });
}
