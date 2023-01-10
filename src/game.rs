#![allow(dead_code, non_snake_case)]

use bevy_ecs::prelude::*;
use rand::prelude::*;

//------------------------------------------------------------------------------
// COMPONENTS
//------------------------------------------------------------------------------
#[derive(Component)]
struct Position {
    // for now, i64 makes some of the math easier although the plan is for
    // the bottom left corner of space to be 0,0 and the top right corner
    // of space to be u32_max, u32_max.. maybe re-examine this later
    x: i64,
    y: i64,
}

#[derive(Component)]
struct Hull {
    max: u32,
    current: u32,
}

#[derive(Component)]
struct Shields {
    max: u32,
    current: u32,
}

#[derive(Component)]
struct Holds {
    max: u32,
    empty: u32,
    fuel: u32,
}

#[derive(Component)]
struct ScannerRange(u32);

#[derive(Debug, Component)]
struct ShipName(String);

#[derive(Debug, Component)]
struct Hangar {
    max: u32,
    current: u32,
}

#[derive(Debug, Component)]
struct DockingPorts(u32);

#[derive(Debug, Component)]
struct CanFitInHangar(bool);

//------------------------------------------------------------------------------
// SHIPS
//------------------------------------------------------------------------------
struct ShipStats {
    hull: u32,
    shields: u32,
    holds: u32,
    hangar_space: u32,
    scanner_range: u32,
    can_fit_in_hangar: bool,
}
trait ShipData {
    fn get_ship_data(&self) -> ShipStats;
}

struct MerchantCruiser;
impl ShipData for MerchantCruiser {
    fn get_ship_data(&self) -> ShipStats {
        ShipStats {
            hull: 100,
            shields: 500,
            holds: 250,
            hangar_space: 0,
            scanner_range: 25,
            can_fit_in_hangar: false,
        }
    }
}

fn create_ship<T: ShipData>(ship_type: &T, world: &mut World, point: Option<Position>) {
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

    world.spawn((
        Position { ..point.unwrap() },
        ShipName(String::from("Merchant Cruiser")),
        Holds { ..holds },
        Shields { ..shields },
        Hull { ..hull },
        ScannerRange { ..scanner_range },
    ));
}

//------------------------------------------------------------------------------
// SYSTEMS
//------------------------------------------------------------------------------
fn print_positions(query: Query<(Entity, &Position)>) {
    for (entity, position) in &query {
        println!(
            "Entity {:?} is at position: x {}, y {}",
            entity, position.x, position.y
        );
    }
}

fn do_ai_scans(
    scanners: Query<(Entity, &Position, &ScannerRange)>,
    scannees: Query<(Entity, &Position)>,
) {
    // TODO stupidly slow prototype code, will have to figure out a smart way to do this at some point
    for (scanner_entity, scanner_position, scanner_range) in scanners.iter() {
        for (scannee_entity, scannee_position) in scannees.iter() {
            if (scanner_entity == scannee_entity) {
                continue;
            }

            // Manhattan distance formula
            let delta_x = (scanner_position.x - scannee_position.x).abs();
            let delta_y = (scanner_position.y - scannee_position.y).abs();
            let distance = delta_x + delta_y;

            if (distance <= scanner_range.0 as i64) {
                // TODO store scanned info in faction data set
                println!(
		            "Entity {:?} at position: x {}, y {} can see entity {:?} at position: x {}, y {}",
		            scanner_entity, scanner_position.x, scanner_position.y, scannee_entity, scannee_position.x, scannee_position.y
		        );
            }
        }
    }
}

//------------------------------------------------------------------------------
// PUBLIC FUNCTIONS
//---------------------------------------------------------------------------------
pub fn init(world: &mut World, schedule: &mut Schedule) -> std::io::Result<()> {
    // TODO attempt to load saved data and bang a new galaxy if there isn't any

    println!("Generating galaxy..");
    for i in 0..5 {
        create_ship(&MerchantCruiser {}, world, None);
    }

    #[derive(StageLabel)]
    pub struct UpdateLabel;

    // TODO add more systems here
    schedule.add_stage(
        UpdateLabel,
        SystemStage::parallel()
            .with_system(do_ai_scans)
            .with_system(print_positions),
    );

    println!("Galaxy Generation complete.");
    Ok(())
}

pub fn tick(mut world: &mut World, schedule: &mut Schedule) -> std::io::Result<()> {
    schedule.run(world);

    Ok(())
}
