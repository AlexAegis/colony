use std::f32::consts::PI;

use bevy::app::App;
use bevy::app::Plugin;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;

/// Spawns a colored axis indicator
#[derive(Default)]
pub struct AxisIndicatorPlugin;

#[derive(Default)]
struct AxisIndicatorResources {
	arrow_body_mesh: Handle<Mesh>,
	arrow_head_mesh: Handle<Mesh>,
	red_material: Handle<StandardMaterial>,
	green_material: Handle<StandardMaterial>,
	blue_material: Handle<StandardMaterial>,
}

#[derive(Component)]
struct AxisIndicatorArrowPart;

impl Plugin for AxisIndicatorPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<AxisIndicatorResources>()
			.add_startup_system(setup_arrow_resources)
			.add_startup_system(spawn_debug_arrow.after(setup_arrow_resources));
	}
}

fn create_arrow_material(
	materials: &mut ResMut<Assets<StandardMaterial>>,
	axis: Vec3,
) -> Handle<StandardMaterial> {
	let h = materials.add(Color::rgba(axis.x, axis.y, axis.z, 0.8).into());
	if let Some(mut mat) = materials.get_mut(&h) {
		mat.metallic = 0.0;
		mat.unlit = true;
		mat.alpha_mode = AlphaMode::Blend;
	}
	h
}

fn create_arrow_mesh(meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
	meshes.add(Mesh::from(shape::Capsule {
		radius: 0.01,
		depth: 1.0,
		..Default::default()
	}))
}

fn create_arrow_head_mesh(meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
	meshes.add(Mesh::from(shape::Capsule {
		radius: 0.04,
		depth: 0.04,
		..Default::default()
	}))
}

fn setup_arrow_resources(
	mut arrow_resoures: ResMut<AxisIndicatorResources>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	arrow_resoures.red_material = create_arrow_material(&mut materials, Vec3::X);
	arrow_resoures.green_material = create_arrow_material(&mut materials, Vec3::Y);
	arrow_resoures.blue_material = create_arrow_material(&mut materials, Vec3::Z);
	arrow_resoures.arrow_body_mesh = create_arrow_mesh(&mut meshes);
	arrow_resoures.arrow_head_mesh = create_arrow_head_mesh(&mut meshes);
}

fn spawn_debug_arrow(mut commands: Commands, arrow_resources: Res<AxisIndicatorResources>) {
	let xt = Transform::from_xyz(0.5, 0.0, 0.0)
		.with_rotation(Quat::from_axis_angle(Vec3::Z, 90.0 * PI / 180.0));
	let yt = Transform::from_xyz(0.0, 0.5, 0.0)
		.with_rotation(Quat::from_axis_angle(Vec3::Y, 90.0 * PI / 180.0));
	let zt = Transform::from_xyz(0.0, 0.0, 0.5)
		.with_rotation(Quat::from_axis_angle(Vec3::X, 90.0 * PI / 180.0));
	commands
		.spawn()
		.insert(AxisIndicatorArrowPart)
		.insert(NotShadowCaster)
		.insert_bundle(PbrBundle {
			mesh: arrow_resources.arrow_body_mesh.clone(),
			material: arrow_resources.red_material.clone(),
			transform: xt,
			..default()
		});
	commands
		.spawn()
		.insert(AxisIndicatorArrowPart)
		.insert(NotShadowCaster)
		.insert_bundle(PbrBundle {
			mesh: arrow_resources.arrow_head_mesh.clone(),
			material: arrow_resources.red_material.clone(),
			transform: xt.with_translation(Vec3::X),
			..default()
		});

	commands
		.spawn()
		.insert(AxisIndicatorArrowPart)
		.insert(NotShadowCaster)
		.insert_bundle(PbrBundle {
			mesh: arrow_resources.arrow_body_mesh.clone(),
			material: arrow_resources.green_material.clone(),
			transform: yt,
			..default()
		});
	commands
		.spawn()
		.insert(AxisIndicatorArrowPart)
		.insert(NotShadowCaster)
		.insert_bundle(PbrBundle {
			mesh: arrow_resources.arrow_head_mesh.clone(),
			material: arrow_resources.green_material.clone(),
			transform: yt.with_translation(Vec3::Y),
			..default()
		});

	commands
		.spawn()
		.insert(AxisIndicatorArrowPart)
		.insert(NotShadowCaster)
		.insert_bundle(PbrBundle {
			mesh: arrow_resources.arrow_body_mesh.clone(),
			material: arrow_resources.blue_material.clone(),
			transform: zt,
			..default()
		});
	commands
		.spawn()
		.insert(AxisIndicatorArrowPart)
		.insert(NotShadowCaster)
		.insert_bundle(PbrBundle {
			mesh: arrow_resources.arrow_head_mesh.clone(),
			material: arrow_resources.blue_material.clone(),
			transform: zt.with_translation(Vec3::Z),
			..default()
		});
}
