use bevy::app::App;
use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::render::camera::Projection;
use bevy::render::camera::ScalingMode;

use crate::game_state::GameState;

#[derive(Default)]
pub struct Player {
	pub entity: Option<Entity>,
	x: f32,
	y: f32,
	move_cooldown: Timer,
}

#[derive(Default)]
pub struct Game {}

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<Player>();
		app.init_resource::<Game>();
		app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_scene));
		app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player));
	}
}

// control the game character
fn move_player(
	mut commands: Commands,
	keyboard_input: Res<Input<KeyCode>>,
	mut player: ResMut<Player>,
	mut transforms: Query<&mut Transform>,
	time: Res<Time>,
) {
	if player.move_cooldown.tick(time.delta()).finished() {
		let mut moved = false;
		let mut rotation = 0.0;

		if keyboard_input.pressed(KeyCode::Up) {
			if player.y < 10.0 {
				player.y += 1.0;
			}
			rotation = -std::f32::consts::FRAC_PI_2;
			moved = true;
		}
		if keyboard_input.pressed(KeyCode::Down) {
			if player.y > -10.0 {
				player.y -= 1.0;
			}
			rotation = std::f32::consts::FRAC_PI_2;
			moved = true;
		}
		if keyboard_input.pressed(KeyCode::Right) {
			if player.x < 10.0 {
				player.x += 1.0;
			}
			rotation = std::f32::consts::PI;
			moved = true;
		}
		if keyboard_input.pressed(KeyCode::Left) {
			if player.x > -10.0 {
				player.x -= 1.0;
			}
			rotation = 0.0;
			moved = true;
		}

		// move on the board
		if moved {
			player.move_cooldown.reset();
			*transforms.get_mut(player.entity.unwrap()).unwrap() = Transform {
				translation: Vec3::new(player.y as f32, 0.5, player.x as f32),
				rotation: Quat::from_rotation_y(rotation),
				..default()
			};
		}
	}
}

/// set up a simple 3D scene
fn setup_scene(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut player: ResMut<Player>,
) {
	info!("hello!");
	// plane
	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
		material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
		..default()
	});
	// player
	player.entity = Some(
		commands
			.spawn_bundle(PbrBundle {
				mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
				material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
				transform: Transform::from_xyz(0.0, 0.5, 0.0),
				..default()
			})
			.id(),
	);

	player.move_cooldown = Timer::from_seconds(0.2, false);

	// light
	commands.spawn_bundle(PointLightBundle {
		point_light: PointLight {
			intensity: 1500.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(4.0, 15.0, 4.0),
		..default()
	});
}
