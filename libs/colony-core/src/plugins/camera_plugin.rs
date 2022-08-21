use bevy::app::App;
use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::render::camera::Projection;
use bevy::render::camera::ScalingMode;

use crate::game_state::GameState;

use super::game_plugin::Player;

pub struct CameraPlugin;

#[derive(Component, Default)]
pub struct CameraData {
	camera_should_focus: Vec3,
	camera_is_focus: Vec3,
}

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_camera));
		app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(focus_camera));
	}
}

// change the focus of the camera
fn focus_camera(
	time: Res<Time>,
	mut player_query: Query<&mut Transform, With<Player>>,
	mut camera: Query<(&mut Transform, &mut CameraData), Without<Player>>,
) {
	const SPEED: f32 = 2.0;
	let (mut camera_transform, mut camera_data) = camera.get_single_mut().unwrap();

	if let Ok(player_transform) = player_query.get_single() {
		camera_data.camera_should_focus = player_transform.translation;
	}
	// calculate the camera motion based on the difference between where the camera is looking
	// and where it should be looking; the greater the distance, the faster the motion;
	// smooth out the camera movement using the frame time
	let mut camera_motion = camera_data.camera_should_focus - camera_data.camera_is_focus;
	if camera_motion.length() > 0.2 {
		camera_motion *= SPEED * time.delta_seconds();
		// set the new camera's actual focus
		camera_data.camera_is_focus += camera_motion;
	}
	// look at that new camera's actual focus
	let mut target = camera_data.camera_is_focus.clone();
	target.y = 20.0;
	target.x = target.x - 20.0;
	camera_transform.translation = target;
	camera_transform.look_at(camera_data.camera_should_focus, Vec3::Y);
}

/// set up a simple 3D scene
fn setup_camera(mut commands: Commands) {
	let ortographic = Projection::Orthographic(OrthographicProjection {
		scale: 0.0125,
		scaling_mode: ScalingMode::WindowSize,
		near: 0.1,
		..Default::default()
	});

	let perspective = Projection::Perspective(PerspectiveProjection {
		near: 0.1,

		..Default::default()
	});
	// camera
	commands
		.spawn_bundle(Camera3dBundle {
			transform: Transform::from_xyz(-2.0, 200.0, 0.25).looking_at(Vec3::ZERO, Vec3::Y),
			projection: perspective,
			..default()
		})
		.insert(CameraData::default());
}
