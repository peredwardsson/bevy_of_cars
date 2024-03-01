pub mod movement;

use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_rapier2d::{prelude::*};
use leafwing_input_manager::InputManagerBundle;

use crate::{
    cars::car_bundle,
    constants::{cars::CAR_SIZE, player::PLAYER_COLOR},
    movement as mvmt};

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {

    // make player_color lime green
    let player_material = MaterialMesh2dBundle {
        material: materials.add(PLAYER_COLOR),
        mesh: Mesh2dHandle(meshes.add(Ellipse::new(CAR_SIZE.x, CAR_SIZE.y))),
        transform: Transform::from_translation(Vec3::new(-250., 0., 0.)),
        ..Default::default()
    };
    commands.spawn(Camera2dBundle::default());
    commands.spawn(car_bundle(player_material))
            .insert(Player)
            .insert(InputManagerBundle::with_map(mvmt::default_map()))
            .insert(ExternalForce::default())
            ;
}