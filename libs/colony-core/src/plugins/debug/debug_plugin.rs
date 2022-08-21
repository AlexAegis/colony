use bevy::app::App;
use bevy::app::Plugin;
#[cfg(debug_assertions)]
use bevy::diagnostic::{
	EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
};

use bevy_rapier3d::prelude::RapierDebugRenderPlugin;

use super::axis_indicator_plugin::AxisIndicatorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
	fn build(&self, _app: &mut App) {
		#[cfg(debug_assertions)]
		{
			_app.add_plugin(FrameTimeDiagnosticsPlugin::default())
				.add_plugin(LogDiagnosticsPlugin::default())
				.add_plugin(EntityCountDiagnosticsPlugin::default())
				.add_plugin(AxisIndicatorPlugin::default());
			//.add_plugin(RapierDebugRenderPlugin::default());
		}
	}
}
