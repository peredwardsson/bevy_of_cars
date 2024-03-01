use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_rapier2d::{prelude::*};
use leafwing_input_manager::prelude::*;

mod movement;
mod player;
mod cars;
mod levels;
mod constants;

use cars::{car_bundle, spawn_ai};
use levels::prelude::*;
use player::{spawn_player, movement::move_player};

const PLAYER_COLOR: Color = Color::rgb(0.25, 0.75, 0.25);
const AI_COLOR: Color = Color::rgb(0.75, 0.25, 0.25);
const PLAYER_MASS: f32 = 100.;

const WINDOW_SIZE: Vec2 = Vec2::new(800., 600.);

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(
                WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WINDOW_SIZE.into(),
                        ..default()
                    }),
                    ..default()
                }
            )
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(InputManagerPlugin::<movement::Actions>::default())
        .add_systems(Startup, (spawn_player, spawn_ai, spawn_walls))
        .add_systems(Update, move_player)
        .run();
}
