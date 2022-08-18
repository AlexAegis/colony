use crate::game_state::GameState;

use bevy::app::Plugin;

use bevy::prelude::{App, ClearColor, Color, Msaa, NonSend, WindowDescriptor};
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;

use super::debug_plugin::DebugPlugin;
use super::loading_plugin::LoadingPlugin;
use std::io::Cursor;
use winit::window::Icon;

pub struct CoreClientPlugin;

impl Plugin for CoreClientPlugin {
	fn build(&self, app: &mut App) {
		app.add_state(GameState::Loading)
			.insert_resource(Msaa { samples: 1 })
			.insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
			.insert_resource(WindowDescriptor {
				width: 1280.,
				height: 720.,
				title: "Colony".to_string(),
				canvas: Some("#bevy".to_owned()),
				..Default::default()
			})
			.add_plugin(LoadingPlugin)
			.add_plugin(DebugPlugin)
			.add_plugins(DefaultPlugins)
			.add_startup_system(set_window_icon);
	}
}

// Sets the icon on windows and X11
fn set_window_icon(windows: NonSend<WinitWindows>) {
	let primary = windows.get_window(WindowId::primary()).unwrap();
	let icon_buf = Cursor::new(include_bytes!("../../../../assets/textures/icon.png"));
	if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
		let image = image.into_rgba8();
		let (width, height) = image.dimensions();
		let rgba = image.into_raw();
		let icon = Icon::from_rgba(rgba, width, height).unwrap();
		primary.set_window_icon(Some(icon));
	};
}
