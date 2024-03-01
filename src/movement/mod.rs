use leafwing_input_manager::prelude::*;
use bevy::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Actions {
    Accelerate,
    Decelerate,
    TurnLeft,
    TurnRight,
    Jump,
}

pub fn default_map() -> InputMap<Actions> {
    InputMap::new(
    vec![
        (Actions::Accelerate, KeyCode::KeyW),
        (Actions::Decelerate, KeyCode::KeyS),
        (Actions::TurnLeft, KeyCode::KeyA),
        (Actions::TurnRight, KeyCode::KeyD),
        (Actions::Jump, KeyCode::Space),
    ]
)
}

pub mod prelude {
    pub use super::Actions;
    pub use super::default_map;
}