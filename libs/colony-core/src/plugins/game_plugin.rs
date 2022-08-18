use bevy::app::App;
use bevy::app::Plugin;
use bevy::prelude::*;

use crate::game_state::GameState;

#[derive(Default)]
struct Player {
	entity: Option<Entity>,
	x: f32,
	y: f32,
	move_cooldown: Timer,
}

#[derive(Default)]
struct Game {
	player: Player,
	camera_should_focus: Vec3,
	camera_is_focus: Vec3,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<Game>();
		app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup));
		app.add_system_set(
			SystemSet::on_update(GameState::Playing)
				.with_system(focus_camera)
				.with_system(move_player),
		);
	}
}

// control the game character
fn move_player(
	mut commands: Commands,
	keyboard_input: Res<Input<KeyCode>>,
	mut game: ResMut<Game>,
	mut transforms: Query<&mut Transform>,
	time: Res<Time>,
) {
	if game.player.move_cooldown.tick(time.delta()).finished() {
		let mut moved = false;
		let mut rotation = 0.0;

		if keyboard_input.pressed(KeyCode::Up) {
			if game.player.y < 10.0 {
				game.player.y += 1.0;
			}
			rotation = -std::f32::consts::FRAC_PI_2;
			moved = true;
		}
		if keyboard_input.pressed(KeyCode::Down) {
			if game.player.y > 0.0 {
				game.player.y -= 1.0;
			}
			rotation = std::f32::consts::FRAC_PI_2;
			moved = true;
		}
		if keyboard_input.pressed(KeyCode::Right) {
			if game.player.x < 10.0 {
				game.player.x += 1.0;
			}
			rotation = std::f32::consts::PI;
			moved = true;
		}
		if keyboard_input.pressed(KeyCode::Left) {
			if game.player.x > 0.0 {
				game.player.x -= 1.0;
			}
			rotation = 0.0;
			moved = true;
		}

		// move on the board
		if moved {
			game.player.move_cooldown.reset();
			*transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
				translation: Vec3::new(game.player.y as f32, 0.5, game.player.x as f32),
				rotation: Quat::from_rotation_y(rotation),
				..default()
			};
		}
	}
}

// change the focus of the camera
fn focus_camera(
	time: Res<Time>,
	mut game: ResMut<Game>,
	mut transforms: ParamSet<(Query<&mut Transform, With<Camera3d>>, Query<&Transform>)>,
) {
	const SPEED: f32 = 2.0;

	if let Some(player_entity) = game.player.entity {
		if let Ok(player_transform) = transforms.p1().get(player_entity) {
			game.camera_should_focus = player_transform.translation;
		}
	}
	// calculate the camera motion based on the difference between where the camera is looking
	// and where it should be looking; the greater the distance, the faster the motion;
	// smooth out the camera movement using the frame time
	let mut camera_motion = game.camera_should_focus - game.camera_is_focus;
	if camera_motion.length() > 0.2 {
		camera_motion *= SPEED * time.delta_seconds();
		// set the new camera's actual focus
		game.camera_is_focus += camera_motion;
	}
	// look at that new camera's actual focus
	for mut transform in transforms.p0().iter_mut() {
		*transform = transform.looking_at(game.camera_is_focus, Vec3::Y);
	}
}

/// set up a simple 3D scene
fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut game: ResMut<Game>,
) {
	info!("hello!");
	// plane
	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
		material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
		..default()
	});
	// cube
	game.player.entity = Some(
		commands
			.spawn_bundle(PbrBundle {
				mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
				material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
				transform: Transform::from_xyz(0.0, 0.5, 0.0),
				..default()
			})
			.id(),
	);

	game.player.move_cooldown = Timer::from_seconds(0.3, false);

	// light
	commands.spawn_bundle(PointLightBundle {
		point_light: PointLight {
			intensity: 1500.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..default()
	});
	// camera
	commands.spawn_bundle(Camera3dBundle {
		transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});
}
