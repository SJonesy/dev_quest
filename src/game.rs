#![allow(dead_code, non_snake_case)]

use bevy_ecs::prelude::*;

// COMPONENTS
#[derive(Component)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Component)]
struct BaseShipStats<'a> {
    name: &'a str,
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

pub fn init(world: &mut World, schedule: &mut Schedule) -> std::io::Result<()> {
    world.spawn((
        Position { x: 1, y: 1 },
        ShipName(String::from("Star Destroyer 1")),
    ));

    world.spawn((
        Position { x: 2, y: 2 },
        ShipName(String::from("Star Destroyer 2")),
    ));

    #[derive(StageLabel)]
    pub struct UpdateLabel;

    schedule.add_stage(
        UpdateLabel,
        SystemStage::parallel().with_system(print_positions),
    );

    Ok(())
}

pub fn tick(mut world: &mut World, schedule: &mut Schedule) -> std::io::Result<()> {
    //schedule.run(&mut world);

    Ok(())
}
