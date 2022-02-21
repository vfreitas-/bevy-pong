use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle {
  position: Vec2,
}

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
  .insert(Paddle { position: position })
  .insert(component);
}
