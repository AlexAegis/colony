use bevy::app::App;
use bevy::app::Plugin;
use bevy::ecs::system::Resource;
use bevy::prelude::*;

use crate::game_state::GameState;

use super::loading_plugin::PlayerModelAssets;

#[derive(Component, Default)]
pub struct Player {
	u: f32,
	v: f32,
	is_running: bool,
	is_speeding_up: bool,
	idle_to_run_cooldown: Timer,
}

#[derive(Default)]
pub struct Game {}

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<Game>();
		app.add_startup_system(load_assets);
		app.add_system_set(
			SystemSet::on_enter(GameState::Playing)
				.with_system(setup_scene)
				.with_system(spawn_player.after(load_assets)),
		);

		app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(player_controller));
	}
}
/*
fn player_controller(
	keyboard_input: Res<Input<KeyCode>>,
	mut animation_player: Query<&mut AnimationPlayer>,
	animations: Res<PlayerAnimations>,
) {
	if let Ok(mut player) = animation_player.get_single_mut() {
		if keyboard_input.just_pressed(KeyCode::Up) {
			let speed = player.speed();
			player.set_speed(speed * 1.2);
		}

		if keyboard_input.just_pressed(KeyCode::Down) {
			let speed = player.speed();
			player.set_speed(speed * 0.8);
		}

		if keyboard_input.just_pressed(KeyCode::Left) {
			let elapsed = player.elapsed();
			player.set_elapsed(elapsed - 0.1);
		}

		if keyboard_input.just_pressed(KeyCode::Right) {
			let elapsed = player.elapsed();
			player.set_elapsed(elapsed + 0.1);
		}

		if keyboard_input.just_pressed(KeyCode::Return) {
			player.play(animations.idle.clone_weak()).repeat();
		}
	}
}
*/
// control the game character
fn player_controller(
	keyboard_input: Res<Input<KeyCode>>,
	mut player_query: Query<(&mut Player, &mut Transform)>,
	mut animation_player_query: Query<&mut AnimationPlayer>,
	player_assets: Res<PlayerAnimations>,
	time: Res<Time>,
) {
	if let Ok((player, player_transform)) = &mut player_query.get_single_mut() {
		if let Ok(mut animation_player) = animation_player_query.get_single_mut() {
			if keyboard_input.pressed(KeyCode::Up) {
				if !player.is_speeding_up && !player.is_running {
					player.is_speeding_up = true;
					animation_player.play(player_assets.idle_to_running.clone());
				}

				if player.is_speeding_up {
					player.idle_to_run_cooldown.tick(time.delta());
				}

				if player.idle_to_run_cooldown.finished() {
					player.idle_to_run_cooldown.reset();
					player.is_speeding_up = false;
					player.is_running = true;
					animation_player
						.play(player_assets.running.clone())
						.repeat();
				}
			} else {
				player.is_speeding_up = false;
				player.is_running = false;
				animation_player.play(player_assets.idle.clone()).repeat();
			}
		}

		if player.is_speeding_up {
			player_transform.translation += Vec3::Z * time.delta_seconds() * 4.0;
		} else if player.is_running {
			player_transform.translation += Vec3::Z * time.delta_seconds() * 8.0;
		}
	}
}

#[derive(Default)]
struct PlayerAnimations {
	character_model: Handle<Scene>,
	idle: Handle<AnimationClip>,
	running: Handle<AnimationClip>,
	idle_to_running: Handle<AnimationClip>,
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
	let character_model = asset_server.load("../../../assets/models/simple_character.glb#Scene0");

	let idle = asset_server.load("../../../assets/models/simple_character.glb#Animation0");
	let running = asset_server.load("../../../assets/models/simple_character.glb#Animation1");
	let idle_to_running =
		asset_server.load("../../../assets/models/simple_character.glb#Animation2");

	commands.insert_resource(PlayerAnimations {
		character_model,
		idle,
		running,
		idle_to_running,
	});
}

fn spawn_player(
	mut commands: Commands,
	player_assets: Res<PlayerAnimations>,
	asset_server: Res<AssetServer>,
) {
	// note that we have to include the `Scene0` label

	// to position our 3d model, simply use the Transform
	// in the SceneBundle
	commands
		.spawn_bundle(SceneBundle {
			scene: player_assets.character_model.clone(),
			transform: Transform::from_xyz(1.0, 0.0, 1.0),
			..Default::default()
		})
		.insert(Player {
			idle_to_run_cooldown: Timer::from_seconds(0.5, false),
			..Default::default()
		});
}

fn setup_player(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut player: ResMut<Player>,
) {
}

/// set up a simple 3D scene
fn setup_scene(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	info!("hello!");
	// plane
	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Plane { size: 15.0 })),
		material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
		..default()
	});
	// player
	// player.entity = Some(
	// 	commands
	// 		.spawn_bundle(PbrBundle {
	// 			mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
	// 			material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
	// 			transform: Transform::from_xyz(0.0, 0.5, 0.0),
	// 			..default()
	// 		})
	// 		.id(),
	// );
	//
	// player.move_cooldown = Timer::from_seconds(0.2, false);

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

	// light
	commands.spawn_bundle(DirectionalLightBundle {
		directional_light: DirectionalLight {
			illuminance: 4000.0,
			color: Color::rgb(1.0, 1.0, 0.8),
			..default()
		},
		transform: Transform::from_xyz(-2.0, 9999.0, -2.0).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});
}
