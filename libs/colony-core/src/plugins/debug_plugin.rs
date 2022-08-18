use bevy::app::App;
use bevy::app::Plugin;
#[cfg(debug_assertions)]
use bevy::diagnostic::{
	EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		#[cfg(debug_assertions)]
		{
			app.add_plugin(FrameTimeDiagnosticsPlugin::default())
				.add_plugin(LogDiagnosticsPlugin::default())
				.add_plugin(EntityCountDiagnosticsPlugin::default());
		}
	}
}
