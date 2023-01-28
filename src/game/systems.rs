use bevy_ecs::prelude::*;

use crate::game::shared_components::*;
use crate::game::resources::*;
use crate::game::ports::*;
use crate::game::ships::*;

pub fn print_positions(query: Query<(Entity, &Position)>) {
    for (entity, position) in &query {
        println!(
            "Entity {:?} is at position: x {}, y {}",
            entity, position.x, position.y
        );
    }
}

pub fn print_holds(turn: Option<Res<Turn>>, query: Query<(Entity, &Holds)>) {
    for (entity, holds) in &query {
        println!("Entity {:?} has the following holds: {:?}", entity, holds);
    }
}

pub fn do_ai_scans(
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

pub fn ports_produce_food(
    turn: Res<Turn>,
    mut ports: Query<(Entity, &Position, &HasFoodModule, &mut Holds)>,
) {
    if (turn.0 % 2 == 0) {
        let turn_num = turn.0;
        return;
    }

    for (port, position, hasFoodModule, mut holds) in &mut ports {
        if (holds.empty > 0) {
            holds.empty -= 1;
            holds.food += 1;
        }
    }
}

pub fn inc_turn(mut turn: ResMut<Turn>) {
    turn.0 += 1;
}

pub fn save(mut turn: Res<Turn>) {
    if (turn.0 % 600 == 0) {
        // TODO save game data
    }
}

pub fn move_ships(mut query: Query<(Entity, &Speed, &mut CurrentAction, &mut Position)>) {
    for (entity, speed, mut action, mut position) in &mut query {
        match action.0 {
            Action::MoveTo(target) => {
               let mut distance_traveled: u32 = 0;
                let mut last_moved_y: bool = true;
                while ((target.x != position.x || target.y != position.y) && distance_traveled < speed.0)  {
                    if (last_moved_y && (target.x != position.x)) {
                        if (target.x > position.x) {
                            position.x += 1;
                        }
                        else {
                            position.x -= 1;
                        }
                        last_moved_y = false;
                    }
                    else if (!last_moved_y && (target.y != position.y)) {
                        if (target.y > position.y) {
                            position.y += 1;
                        }
                        else {
                            position.y -= 1;
                        }
                        last_moved_y = true;
                    }
                    else {
                        // TODO this is the most concise way I can think of to do this
                        // logic, but this thrashes when we're just moving on X or Y
                        // by itself. Maybe rewrite all of this? Writing it the most
                        // naive way with lots of if statements might be faster.
                        last_moved_y = !last_moved_y;
                    }
                    distance_traveled += 1;
                }
                if (target.x == position.x && target.y == position.y) {
                    println!("Entity {:?} arrived at position: x {}, y {}",
                        entity, position.x, position.y);
                    action.0 = Action::Idle;
                    break;
                }
                else {
                    println!("Entity {:?} moved to position: x {}, y {}",
                        entity, position.x, position.y);
                }
             }
            _ => {} 
        }
    }
}


