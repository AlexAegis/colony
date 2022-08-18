// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use colony_core::{plugins::core_client_plugin::CoreClientPlugin, App};

fn main() {
	App::new().add_plugin(CoreClientPlugin).run();
}
