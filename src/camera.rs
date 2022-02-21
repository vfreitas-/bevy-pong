use bevy::{prelude::*, render::camera::{ScalingMode, DepthCalculation, WindowOrigin}};

pub fn setup_camera (
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