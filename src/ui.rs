use bevy::prelude::*;
pub struct UIPlugin;

impl Plugin for UIPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(hud_setup);
  }
}

fn hud_setup (
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  commands.spawn_bundle(
    UiCameraBundle::default()
  );

  commands.spawn_bundle(
    NodeBundle {
      style: Style {
        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        flex_direction: FlexDirection::ColumnReverse,
        ..Default::default()
      },
      color: Color::NONE.into(),
      ..Default::default()
    }
  )
  .with_children(|parent| {
    parent.spawn_bundle(
      NodeBundle {
        style: Style {
          size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
          justify_content: JustifyContent::SpaceAround,
          align_items: AlignItems::Center,
          ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
      }
    )
    .with_children(|parent| {
      parent.spawn_bundle(
        TextBundle {
          text: Text::with_section(
            "00".to_string(),
            TextStyle {
              color: Color::WHITE,
              font_size: 80.,
              font: asset_server.load("./Anton-Regular.ttf"),
              ..Default::default()
            },
            TextAlignment {
              ..Default::default()
            }
          ),
          ..Default::default()
        }
      );
      parent.spawn_bundle(
        TextBundle {
          text: Text::with_section(
            " Pong! ".to_string(),
            TextStyle {
              color: Color::WHITE,
              font_size: 96.,
              font: asset_server.load("./Anton-Regular.ttf"),
              ..Default::default()
            },
            TextAlignment {
              ..Default::default()
            }
          ),
          ..Default::default()
        }
      );
      parent.spawn_bundle(
        TextBundle {
          text: Text::with_section(
            "00".to_string(),
            TextStyle {
              color: Color::WHITE,
              font_size: 80.,
              font: asset_server.load("./Anton-Regular.ttf"),
              ..Default::default()
            },
            TextAlignment {
              ..Default::default()
            }
          ),
          ..Default::default()
        }
      );
    });
  });
}