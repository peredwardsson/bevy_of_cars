use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::constants::{app::WINDOW_SIZE, levels::*};


#[derive(Copy, Clone)]
enum WallLocation {
    Top,
    Bottom,
    Left,
    Right,
}

impl Into<Transform> for WallLocation {
    fn into(self) -> Transform {
        match self {
            WallLocation::Top => Transform::from_translation(Vec3::new(0., WINDOW_SIZE.y / 2., 0.)),
            WallLocation::Bottom => Transform::from_translation(Vec3::new(0., -WINDOW_SIZE.y / 2., 0.)),
            WallLocation::Left => Transform::from_translation(Vec3::new(-WINDOW_SIZE.x / 2., 0., 0.)),
            WallLocation::Right => Transform::from_translation(Vec3::new(WINDOW_SIZE.x / 2., 0., 0.)),
        }
    }
}

impl WallLocation {
    fn size(&self) -> Vec2 {
        match self {
            WallLocation::Top | WallLocation::Bottom => Vec2::new(WINDOW_SIZE.x, WALL_WIDTH),
            WallLocation::Left | WallLocation::Right => Vec2::new(WALL_WIDTH, WINDOW_SIZE.y),
        }
    }
}

#[derive(Component)]
struct Wall{
    location: WallLocation,
}

impl Wall {
    fn new(location: WallLocation) -> Self {
        Self {
            location,
        }
    }

}

pub fn spawn_walls(
mut commands: Commands
) {
    for location in [WallLocation::Top, WallLocation::Bottom, WallLocation::Left, WallLocation::Right].iter() {
        let wall = Wall::new(*location);
        let transform: Transform = (*location).into();
        let size = location.size();
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(size),
                    ..default()
                },
                transform,
                ..default()
            },
            Collider::cuboid(size.x / 2., size.y / 2.),
            wall,
        ));
    }
}