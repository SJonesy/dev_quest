#![allow(dead_code, non_snake_case)]

use bevy_ecs::prelude::*;

// COMPONENTS
#[derive(Component)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Component)]
struct BaseShipStats {
    id: u32,
    max_shields: u32,
    max_holds: u32,
    max_hull: u32,
    scanner_range: u32,
}

#[derive(Component)]
struct ActiveShipData {
    current_holds_empty: u32,
    current_holds_fuel: f32,
    current_hull: u32,
    current_shields: u32,
}

#[derive(Debug, Component)]
struct ShipName(String);

// SYSTEMS
fn print_positions(query: Query<(Entity, &Position)>) {
    for (entity, position) in &query {
        println!(
            "Entity {:?} is at position: x {}, y {}",
            entity, position.x, position.y
        );
    }
}

fn do_ai_scans(
    scanners: Query<(Entity, &Position, &BaseShipStats)>,
    scannees: Query<(Entity, &Position)>,
) {
    for (scanner_entity, scanner_position, base_ship_stats) in scanners.iter() {
        for (scannee_entity, scannee_position) in scannees.iter() {
            // TODO SCJ stupidly slow prototype code, will have to figure out a smart way to do this at some point
            let delta_x = ((scanner_position.x - scannee_position.x) as f64)
                .abs()
                .powf(2.0);
            let delta_y = ((scanner_position.y - scannee_position.y) as f64)
                .abs()
                .powf(2.0);
            let distance = f64::sqrt(delta_x + delta_y);

            if (distance <= base_ship_stats.scanner_range as f64) {
                // TODO store scanned info in faction data set
                println!(
		            "Entity {:?} at position: x {}, y {} can see entity {:?} at position: x {}, y {}",
		            scanner_entity, scanner_position.x, scanner_position.y, scannee_entity, scannee_position.x, scannee_position.y
		        );
            }
        }
    }
}

pub fn init(world: &mut World, schedule: &mut Schedule) -> std::io::Result<()> {
    // TODO attempt to load saved data and bang a new galaxy if there isn't any

    let merchant_cruiser = BaseShipStats {
        id: 0,
        max_shields: 500,
        max_holds: 1000,
        max_hull: 100,
        scanner_range: 25,
    };

    let merchant_cruiser_starting = ActiveShipData {
        current_holds_empty: merchant_cruiser.max_holds,
        current_holds_fuel: 0.0 as f32,
        current_hull: merchant_cruiser.max_hull,
        current_shields: merchant_cruiser.max_shields,
    };

    world.spawn((
        Position { x: 1, y: 1 },
        ShipName(String::from("Pirate Merchant 1")),
        BaseShipStats { ..merchant_cruiser },
        ActiveShipData {
            ..merchant_cruiser_starting
        },
    ));

    world.spawn((
        Position { x: 2, y: 2 },
        ShipName(String::from("Pirate Merchant 2")),
        BaseShipStats { ..merchant_cruiser },
        ActiveShipData {
            ..merchant_cruiser_starting
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
    schedule.run(&mut world);

    Ok(())
}
