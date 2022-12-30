#![allow(dead_code, non_snake_case)]

use bevy_ecs::prelude::*;

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
            let delta_x = ((scanner_position.x - scannee_position.x) as f64)
                .abs()
                .powf(2.0);
            let delta_y = ((scanner_position.y - scannee_position.y) as f64)
                .abs()
                .powf(2.0);
            let distance = f64::sqrt(delta_x + delta_y);

            if (distance <= scanner_range.0 as f64) {
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
    let merchant_cruiser_shields = Shields {
        max: 500,
        current: 500,
    };

    let merchant_cruiser_hull = Hull {
        max: 100,
        current: 100,
    };

    let merchant_cruiser_holds = Holds {
        max: 1000,
        empty: 1000,
        fuel: 0,
    };

    let merchant_cruiser_scanner_range = ScannerRange(25);

    world.spawn((
        Position { x: 1, y: 1 },
        ShipName(String::from("Merchant Cruiser")),
        Holds {
            ..merchant_cruiser_holds
        },
        Shields {
            ..merchant_cruiser_shields
        },
        Hull {
            ..merchant_cruiser_hull
        },
        ScannerRange {
            ..merchant_cruiser_scanner_range
        },
    ));

    world.spawn((
        Position { x: 2, y: 2 },
        ShipName(String::from("Merchant Cruiser")),
        Holds {
            ..merchant_cruiser_holds
        },
        Shields {
            ..merchant_cruiser_shields
        },
        Hull {
            ..merchant_cruiser_hull
        },
        ScannerRange {
            ..merchant_cruiser_scanner_range
        },
    ));

    #[derive(StageLabel)]
    pub struct UpdateLabel;

    schedule.add_stage(
        UpdateLabel,
        SystemStage::parallel().with_system(do_ai_scans),
    );

    Ok(())
}

pub fn tick(mut world: &mut World, schedule: &mut Schedule) -> std::io::Result<()> {
    schedule.run(world);

    Ok(())
}
