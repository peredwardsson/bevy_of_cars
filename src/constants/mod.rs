
// App related
pub(crate) mod app {
    use bevy::prelude::*;
    pub const WINDOW_SIZE: Vec2 = Vec2::new(800., 600.);
}

// Level related
pub(crate) mod levels {
    use bevy::prelude::*;
    pub const WALL_WIDTH: f32 = 10.;
    pub const WALL_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
}

// Cars
pub(crate) mod cars {
    use bevy::prelude::*;
    pub const ENGINE_FORCE: f32 = 450.;
    pub const TURNING_SPEED: f32 = 1.;
    pub const CAR_SIZE: Vec2 = Vec2::new(50., 20.);
    pub const CAR_MASS: f32 = 100.;

    // AI
    pub const AI_COLOR: Color = Color::rgb(0.75, 0.25, 0.25);
}

// Player
pub(crate) mod player {
    use bevy::prelude::*;
    pub const PLAYER_COLOR: Color = Color::rgb(0.25, 0.75, 0.25);
    pub const PLAYER_MASS: f32 = 100.;
}

