use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(level_setup);
  }
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
    );

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
    );
}
