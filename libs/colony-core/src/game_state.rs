/// https://bevy-cheatbook.github.io/programming/states.html
/// https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
	Loading,
	Playing,
}
