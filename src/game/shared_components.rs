use bevy_ecs::prelude::*;

#[derive(PartialEq)]
pub enum Action {
    Idle,
    DockAt(Entity),
    MoveTo(Position),
}
#[derive(Component)]
pub struct CurrentAction(pub Action);

#[derive(Component, PartialEq, Copy, Clone)]
pub struct Position {
    // for now, i64 makes some of the math easier although the plan is for
    // the bottom left corner of space to be 0,0 and the top right corner
    // of space to be u32_max, u32_max.. maybe re-examine this later
    pub x: i64,
    pub y: i64,
}

#[derive(Component)]
pub struct Hull {
    pub max: u32,
    pub current: u32,
}

#[derive(Component)]
pub struct Shields {
    pub max: u32,
    pub current: u32,
}

#[derive(Debug, Component)]
pub struct Holds {
    pub max: u32,
    pub empty: u32,
    pub fuel: u32,
    pub food: u32,
}

#[derive(Component)]
pub struct ScannerRange(pub u32);

#[derive(Debug, Component)]
pub struct Hangar {
    pub max: u32,
    pub current: u32,
}

#[derive(Debug, Component)]
pub struct Name(pub String);
