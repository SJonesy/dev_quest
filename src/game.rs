#![allow(dead_code, non_snake_case)]

use bevy_ecs::prelude::*;

// COMPONENTS
#[derive(Debug, Component)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Debug, Component)]
struct Name(String);

// SYSTEMS
fn print_positions(query: Query<(Entity, &Position, &Name)>) {
    for (entity, position, name) in &query {
        println!(
            "Entity \"{:?}\" is at position: x {}, y {}",
            name, position.x, position.y
        );
    }
}

pub fn init(world: &mut World, schedule: &mut Schedule) -> std::io::Result<()> {
    world.spawn((
        Position { x: 1, y: 1 },
        Name(String::from("Star Destroyer 1")),
    ));

    world.spawn((
        Position { x: 2, y: 2 },
        Name(String::from("Star Destroyer 2")),
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
