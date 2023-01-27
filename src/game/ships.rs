use crate::game::shared_components::*;
use bevy_ecs::prelude::*;
use bevy_ecs::world::EntityMut;
use rand::prelude::*;

//------------------------------------------------------------------------------
// COMPONENTS
//------------------------------------------------------------------------------
#[derive(Debug, Component)]
struct CanFitInHangar(bool);

#[derive(Debug, Component)]
pub struct Speed(pub u32);

//------------------------------------------------------------------------------
// SHIPS
//------------------------------------------------------------------------------
pub struct ShipStats {
    name: String,
    hull: u32,
    shields: u32,
    holds: u32,
    hangar_space: u32,
    scanner_range: u32,
    can_fit_in_hangar: bool,
    speed: u32,
}
pub trait ShipData {
    fn get_ship_data(&self) -> ShipStats;
}

pub struct MerchantCruiser;
impl ShipData for MerchantCruiser {
    fn get_ship_data(&self) -> ShipStats {
        ShipStats {
            name: String::from("Merchant Cruiser"),
            hull: 100,
            shields: 500,
            holds: 250,
            hangar_space: 0,
            scanner_range: 500,
            can_fit_in_hangar: false,
            speed: 10
        }
    }
}

//------------------------------------------------------------------------------
// FUNCTIONS
//------------------------------------------------------------------------------
pub fn create_ship<'a, T: ShipData>(ship_type: &'a T, world: &'a mut World, point: Option<Position>) -> EntityMut<'a> {
    let ship_stats = ship_type.get_ship_data();

    let shields = Shields {
        max: ship_stats.shields,
        current: ship_stats.shields,
    };

    let hull = Hull {
        max: ship_stats.hull,
        current: ship_stats.hull,
    };

    let holds = Holds {
        max: ship_stats.hull,
        empty: ship_stats.hull,
        fuel: 0,
        food: 0,
    };

    let scanner_range = ScannerRange(ship_stats.scanner_range);

    let mut point = point;
    if let None = point {
        let mut rng = rand::thread_rng();
        point = Some(Position {
            x: rng.gen::<u32>() as i64,
            y: rng.gen::<u32>() as i64,
        });
    }

    let current_action = CurrentAction(Action::Idle);

    let speed = Speed(ship_stats.speed);

    world.spawn((
        Position { ..point.unwrap() },
        Name(ship_stats.name),
        Holds { ..holds },
        Shields { ..shields },
        Hull { ..hull },
        ScannerRange { ..scanner_range },
        CurrentAction { ..current_action },
        Speed { ..speed },
    ))
}
