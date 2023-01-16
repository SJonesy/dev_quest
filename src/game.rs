#![allow(dead_code, non_snake_case)]

use bevy_ecs::{component::ComponentTicks, prelude::*};
use rand::prelude::*;

//------------------------------------------------------------------------------
// RESOURCES
//------------------------------------------------------------------------------
#[derive(Resource, Default)]
struct Turn(usize);

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

#[derive(Debug, Component)]
struct Holds {
    max: u32,
    empty: u32,
    fuel: u32,
    food: u32,
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

#[derive(Debug, Component)]
struct HasFoodModule(bool);

//------------------------------------------------------------------------------
// SHIPS
//------------------------------------------------------------------------------
struct ShipStats {
    name: String,
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
            name: String::from("Merchant Cruiser"),
            hull: 100,
            shields: 500,
            holds: 250,
            hangar_space: 0,
            scanner_range: 500,
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

    world.spawn((
        Position { ..point.unwrap() },
        ShipName(ship_stats.name),
        Holds { ..holds },
        Shields { ..shields },
        Hull { ..hull },
        ScannerRange { ..scanner_range },
    ));
}

//------------------------------------------------------------------------------
// PORTS
//------------------------------------------------------------------------------
enum PortSize {
    Outpost,
    Starport,
    Starhold,
    StarFortress,
    Citadel,
}
#[derive(PartialEq)]
enum PortModule {
    Food,
}
struct PortStats {
    name: String,
    modules: Vec<PortModule>,
    size: PortSize,
    scanner_range: u32,
    shields: u32,
    docking_ports: u32,
}
trait PortData {
    fn get_port_data(&self) -> PortStats;
}

struct FoodFactory;
impl PortData for FoodFactory {
    fn get_port_data(&self) -> PortStats {
        PortStats {
            name: String::from("Food Factory"),
            modules: vec![PortModule::Food],
            size: PortSize::Starport,
            scanner_range: 10000,
            shields: 20000,
            docking_ports: 24,
        }
    }
}
fn create_port<T: PortData>(port_type: &T, world: &mut World, point: Option<Position>) {
    let port_stats = port_type.get_port_data();

    let shields = Shields {
        max: port_stats.shields,
        current: port_stats.shields,
    };

    let mut hull = Hull { max: 0, current: 0 };

    let mut holds = Holds {
        max: 0,
        empty: 0,
        fuel: 0,
        food: 0,
    };

    // TODO some better values
    match port_stats.size {
        PortSize::Outpost => {
            hull.max = 10000;
            holds.max = 50000;
        }
        PortSize::Starport => {
            hull.max = 10000;
            holds.max = 50000;
        }
        PortSize::Starhold => {
            hull.max = 10000;
            holds.max = 50000;
        }
        PortSize::StarFortress => {
            hull.max = 10000;
            holds.max = 50000;
        }
        PortSize::Citadel => {
            hull.max = 10000;
            holds.max = 50000;
        }
    }
    hull.current = hull.max;
    holds.empty = holds.max;

    let mut point = point;
    if let None = point {
        let mut rng = rand::thread_rng();
        point = Some(Position {
            x: rng.gen::<u32>() as i64,
            y: rng.gen::<u32>() as i64,
        });
    }

    let mut foodModule = false;
    for module in port_stats.modules {
        if (module == PortModule::Food) {
            foodModule = true;
        }
    }

    world.spawn((
        Position { ..point.unwrap() },
        ShipName(port_stats.name),
        Holds { ..holds },
        Shields { ..shields },
        Hull { ..hull },
        ScannerRange {
            ..ScannerRange(port_stats.scanner_range)
        },
        DockingPorts {
            ..DockingPorts(port_stats.docking_ports)
        },
        HasFoodModule {
            ..HasFoodModule(foodModule)
        },
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

    println!("Galaxy Generation complete.");
    Ok(())
}

pub fn tick(mut world: &mut World, schedule: &mut Schedule) -> std::io::Result<()> {
    schedule.run(world);

    Ok(())
}
