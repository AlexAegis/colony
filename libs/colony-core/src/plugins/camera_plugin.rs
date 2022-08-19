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
	player: Res<Player>,
	mut camera: Query<(&mut Transform, &mut CameraData)>,
	transforms: Query<&Transform, Without<CameraData>>,
) {
	const SPEED: f32 = 2.0;
	let (mut camera_transform, mut camera_data) = camera.get_single_mut().unwrap();

	if let Some(player_entity) = player.entity {
		if let Ok(player_transform) = transforms.get(player_entity) {
			camera_data.camera_should_focus = player_transform.translation;
		}
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
	target.y = 5.0;
	camera_transform.translation = target;
}

/// set up a simple 3D scene
fn setup_camera(mut commands: Commands) {
	// camera
	commands
		.spawn_bundle(Camera3dBundle {
			transform: Transform::from_xyz(-1.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
			projection: Projection::Orthographic(OrthographicProjection {
				scale: 0.0125,
				scaling_mode: ScalingMode::WindowSize,
				..Default::default()
			}),
			..default()
		})
		.insert(CameraData::default());
}
