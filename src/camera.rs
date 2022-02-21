use bevy::{prelude::*, render::camera::{ScalingMode, DepthCalculation, WindowOrigin}};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(camera_setup);
  }
}

fn camera_setup (
  mut commands: Commands,
  mut windows: ResMut<Windows>,
) {
  let mut camera = OrthographicCameraBundle::new_2d();
  camera.orthographic_projection = OrthographicProjection {
    depth_calculation: DepthCalculation::ZDifference,
    scaling_mode: ScalingMode::FixedVertical,
    window_origin: WindowOrigin::Center,
    scale: 20.0,
    ..Default::default()
  };
  commands.spawn_bundle(camera);

  let window = windows.get_primary_mut().unwrap();
  window.set_position(IVec2::new(1750, 200));
}