use bevy::{prelude::*, render::mesh::shape::Cube, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_rapier2d::{dynamics::RigidBody, prelude::* };
use crate::constants::cars::*;

// pub const CAR_MASS: f32 = 100.;
// pub const ENGINE_FORCE: f32 = 450.;

pub fn car_bundle(
    material: MaterialMesh2dBundle::<ColorMaterial>,
) -> impl Bundle {
    return (
        RigidBody::Dynamic,
        // AdditionalMassProperties::Mass(CAR_MASS),
        material,
        Velocity::zero(),
        Collider::capsule_y(CAR_SIZE.x / 2., CAR_SIZE.y / 2.),
        GravityScale(0.),
        
    )
}

pub fn spawn_ai(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let ai_bundle = MaterialMesh2dBundle {
        material: materials.add(AI_COLOR),
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(CAR_SIZE.x, CAR_SIZE.y))),
        transform: Transform::from_translation(Vec3::new(250., 0., 0.)),
        ..Default::default()
    };
    commands.spawn(car_bundle(ai_bundle));
}
