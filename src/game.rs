#![allow(dead_code, non_snake_case)]

use specs::prelude::*;

// COMPONENT DECLARATIONS
#[derive(Debug)]
struct X(i32);
#[derive(Debug)]
struct Y(i32);
#[derive(Debug)]
struct Name(String);

// COMPONENT IMPLEMENTATIONS
impl Component for X {
    type Storage = VecStorage<Self>;
}
impl Component for Y {
    type Storage = VecStorage<Self>;
}
impl Component for Name {
    type Storage = VecStorage<Self>;
}

// SYSTEM DECLARATIONS
struct OutputObjects;

// SYSTEM IMPLEMENTATIONS
impl<'a> System<'a> for OutputObjects {
    type SystemData = (
        ReadStorage<'a, X>,
        ReadStorage<'a, Y>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, (x, y, name): Self::SystemData) {
        for (x, y, name) in (&x, &y, &name).join() {
            println!("{:?} is at {:?}, {:?}", name, x, y);
        }
    }
}

pub fn init(world: &mut specs::World) -> std::io::Result<()> {
    world.register::<X>();
    world.register::<Y>();
    world.register::<Name>();

    world
        .create_entity()
        .with(X(1))
        .with(Y(1))
        .with(Name(String::from("Star Destroyer")))
        .build();

    world
        .create_entity()
        .with(X(2))
        .with(Y(2))
        .with(Name(String::from("Star Destroyer 2")))
        .build();

    Ok(())
}

pub fn tick(world: &mut World) -> std::io::Result<()> {
    OutputObjects.run_now(&world);
    world.maintain();

    Ok(())
}
