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


    world.spawn((
        Position { ..point.unwrap() },
        Name(port_stats.name),
        Holds { ..holds },
        Shields { ..shields },
        Hull { ..hull },
        ScannerRange {
            ..ScannerRange(port_stats.scanner_range)
        },
        DockingPorts {
            ..DockingPorts(port_stats.docking_ports)
        },
        HasFoodModule {
            ..HasFoodModule(foodModule)
        },
    ));
}
