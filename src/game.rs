#![allow(dead_code, non_snake_case)]

mod planets;
mod ports;
mod shared_components;
mod ships;
mod systems;
mod resources;

use crate::game::planets::*;
use crate::game::ports::*;
use crate::game::shared_components::*;
use crate::game::ships::*;
use crate::game::systems::*;
use crate::game::resources::*;

use bevy_ecs::prelude::*;
use rand::prelude::*;

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
    let mut ship = create_ship(&MerchantCruiser {}, world, Some(Position { x: 400, y: 60 }));
    let port_position = Position { x: 420, y: 69 };
    ship.insert(CurrentAction(Action::MoveTo(port_position)));

    world.insert_resource(Turn(0));

    #[derive(StageLabel)]
    pub struct Main;

    schedule.add_stage(
        Main,
        SystemStage::parallel()
            //.with_system(do_ai_scans)
            //.with_system(print_holds)
            .with_system(ports_produce_food) 
            .with_system(move_ships)
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
