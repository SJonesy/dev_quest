use crate::game::shared_components::*;
use bevy_ecs::prelude::*;
use rand::prelude::*;

//------------------------------------------------------------------------------
// COMPONENTS
//------------------------------------------------------------------------------
#[derive(Debug, Component)]
pub struct OrganicProduction(pub u32);

//------------------------------------------------------------------------------
// PLANETS
//------------------------------------------------------------------------------
pub enum PlanetType {
    Terran,
    RockyMetallic,
    Volcanic,
    Water,
    Desert,
    Arid,
    Tundra,
    Frozen,
    Barren
}

pub enum PlanetSize {
    Tiny,
    Small,
    Medium,
    Large,
    Huge
}

pub struct PlanetStats {
    pub name: String,
    pub size: PlanetSize,
    pub planet_type: PlanetType,
}

//------------------------------------------------------------------------------
// FUNCTIONS
//------------------------------------------------------------------------------
pub fn create_planet(world: &mut World, planet_type: PlanetType, point: Option<Position>, size: Option<PlanetSize>) {
    let mut point = point;
    if let None = point {
        let mut rng = rand::thread_rng();
        point = Some(Position {
            x: rng.gen::<u32>() as i64,
            y: rng.gen::<u32>() as i64,
        });
    }

    let mut size = size;
    if let None = size {
        let mut rng = rand::thread_rng();
        size = match rng.gen_range(0..4) {
            0 => Some(PlanetSize::Tiny),
            1 => Some(PlanetSize::Small),
            2 => Some(PlanetSize::Medium),
            3 => Some(PlanetSize::Large),
            4 => Some(PlanetSize::Huge)
        };
    }

    world.spawn((
        Position { ..point.unwrap() },
        Size { ..size.unwrap() },
        
    ));
}
