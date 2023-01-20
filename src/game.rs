#![allow(dead_code, non_snake_case)]

mod ports;
mod shared_components;
mod ships;

use crate::game::ports::*;
use crate::game::shared_components::*;
use crate::game::ships::*;

use bevy_ecs::{component::ComponentTicks, prelude::*};
use rand::prelude::*;

//------------------------------------------------------------------------------
// RESOURCES
//------------------------------------------------------------------------------
#[derive(Resource, Default)]
struct Turn(usize);

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

fn print_holds(turn: Option<Res<Turn>>, query: Query<(Entity, &Holds)>) {
    for (entity, holds) in &query {
        println!("Entity {:?} has the following holds: {:?}", entity, holds);
    }
}

fn do_ai_scans(
    turn: Option<Res<Turn>>,
    scanners: Query<(Entity, &Position, &ScannerRange)>,
    scannees: Query<(Entity, &Position)>,
) {
    // TODO stupidly slow prototype code, must figure out a smart way to do this at some point
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

fn ports_produce_food(
    turn: Res<Turn>,
    mut ports: Query<(Entity, &Position, &HasFoodModule, &mut Holds)>,
) {
    if (turn.0 % 2 == 0) {
        let turn_num = turn.0;
        println!("Skipping on {turn_num}");
        return;
    }
    for (port, position, hasFoodModule, mut holds) in &mut ports {
        if (holds.empty > 0) {
            holds.empty -= 1;
            holds.food += 1;
        }
    }
}

fn inc_turn(mut turn: ResMut<Turn>) {
    turn.0 += 1;
}

fn save(mut turn: Res<Turn>) {
    if (turn.0 % 600 == 0) {
        // TODO save game data
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

    // TODO temporary entities for starting to test docking/purchasing
    create_port(&FoodFactory {}, world, Some(Position { x: 420, y: 69 }));
    create_ship(&MerchantCruiser {}, world, Some(Position { x: 400, y: 60 }));

    world.insert_resource(Turn(0));

    #[derive(StageLabel)]
    pub struct Main;

    schedule.add_stage(
        Main,
        SystemStage::parallel()
            .with_system(do_ai_scans)
            .with_system(print_holds)
            .with_system(ports_produce_food), // .with_system(print_positions)
    );

    #[derive(StageLabel)]
    pub struct PostMain;
    schedule.add_stage(PostMain, SystemStage::parallel().with_system(inc_turn));

    #[derive(StageLabel)]
    pub struct Cleanup;
    schedule.add_stage(Cleanup, SystemStage::parallel().with_system(save));

    println!("Galaxy Generation complete.");
    Ok(())
}

pub fn tick(mut world: &mut World, schedule: &mut Schedule) -> std::io::Result<()> {
    schedule.run(world);

    Ok(())
}
