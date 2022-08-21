use crate::game_state::GameState;

use bevy::app::Plugin;

use bevy::log::LogSettings;
use bevy::prelude::{App, ClearColor, Color, Msaa, NonSend, WindowDescriptor};
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;

use super::camera_plugin::CameraPlugin;
use super::debug::debug_plugin::DebugPlugin;
use super::game_plugin::GamePlugin;
use super::loading_plugin::LoadingPlugin;
use super::physics_plugin::PhysicsPlugin;
#[cfg(target_family = "wasm")]
use bevy_web_fullscreen::FullViewportPlugin;
use std::io::Cursor;
use winit::window::Icon;

pub struct CoreClientPlugin;

impl Plugin for CoreClientPlugin {
	fn build(&self, app: &mut App) {
		// this code is compiled only if debug assertions are enabled (debug mode)
		#[cfg(debug_assertions)]
		app.insert_resource(LogSettings {
			filter: "info,wgpu_core=warn,wgpu_hal=warn,minewars=debug".into(),
			level: bevy::log::Level::DEBUG,
		});

		// this code is compiled only if debug assertions are disabled (release mode)
		#[cfg(not(debug_assertions))]
		app.insert_resource(LogSettings {
			filter: "warn".into(),
			level: bevy::log::Level::WARN,
		});

		#[cfg(target_family = "wasm")]
		app.add_plugin(FullViewportPlugin);

		app.add_state(GameState::Playing)
			.insert_resource(Msaa { samples: 4 })
			.insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
			.insert_resource(WindowDescriptor {
				width: 1280.,
				height: 720.,
				title: "Colony".to_string(),
				canvas: Some("#bevy".to_owned()),
				..Default::default()
			})
			.add_plugin(LoadingPlugin)
			.add_plugin(GamePlugin)
			.add_plugin(CameraPlugin)
			.add_plugin(DebugPlugin)
			.add_plugins(DefaultPlugins)
			.add_system(bevy::window::close_on_esc)
			.add_startup_system(set_window_icon);
	}
}

// Sets the icon on windows and X11
fn set_window_icon(windows: NonSend<WinitWindows>) {
	let primary = windows.get_window(WindowId::primary()).unwrap();
	let icon_buf = Cursor::new(include_bytes!("../../../../assets/textures/bevy.png"));
	if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
		let image = image.into_rgba8();
		let (width, height) = image.dimensions();
		let rgba = image.into_raw();
		let icon = Icon::from_rgba(rgba, width, height).unwrap();
		primary.set_window_icon(Some(icon));
	};
}
