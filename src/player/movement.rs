use leafwing_input_manager::prelude::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::Player;
use crate::{
    movement::prelude::*,
    constants::cars::{ENGINE_FORCE, TURNING_SPEED},
};

pub fn move_player(
    mut velocities: Query<&mut Velocity, With<Player>>,
    mut force: Query<&mut ExternalForce, With<Player>>,
    query: Query<&ActionState<Actions>, With<Player>>,
) {
    let Ok(mut vel) = velocities.get_single_mut() else {
        error!("Unable to find player velocity!");
        return;
    };
    let Ok(mut extf) = force.get_single_mut() else {
        error!("Unable to find player force!");
        return;
    };
    let Ok(action) = query.get_single() else {
        error!("Unable to find player action state!");
        return;
    };
    use Actions::*;
    if action.pressed(&Accelerate) {
        extf.force = Vec2::new(ENGINE_FORCE, 0.);
    } else if action.pressed(&Decelerate) {
        extf.force = Vec2::new(-ENGINE_FORCE, 0.);
    } else if action.pressed(&TurnLeft) {
        vel.angvel = TURNING_SPEED;
    } else if action.pressed(&TurnRight) {
        vel.angvel = -TURNING_SPEED;
    } else {
        extf.force = Vec2::ZERO;
        vel.angvel = 0.;
    }
}