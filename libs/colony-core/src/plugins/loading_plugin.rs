use crate::game_state::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
	fn build(&self, app: &mut App) {
		app.add_loading_state(
			LoadingState::new(GameState::Loading)
				.with_collection::<PlayerModelAssets>()
				.continue_to_state(GameState::Playing),
		);
	}
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
	#[asset(path = "../../../assets/fonts/FiraSans-Bold.ttf")]
	pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct PlayerModelAssets {
	#[asset(path = "../../../assets/models/simple_character.glb#Scene0")]
	pub character_model: Handle<Scene>,
	#[asset(path = "../../../assets/models/simple_character.glb#Animation0")]
	pub idle: Handle<AnimationClip>,
	#[asset(path = "../../../assets/models/simple_character.glb#Animation1")]
	pub running: Handle<AnimationClip>,
	#[asset(path = "../../../assets/models/simple_character.glb#Animation2")]
	pub idle_to_running: Handle<AnimationClip>,
}
