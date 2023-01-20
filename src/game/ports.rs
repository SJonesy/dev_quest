use crate::game::shared_components::*;
use bevy_ecs::prelude::*;
use rand::prelude::*;

//------------------------------------------------------------------------------
// COMPONENTS
//------------------------------------------------------------------------------
#[derive(Debug, Component)]
pub struct HasFoodModule(pub bool);

#[derive(Debug, Component)]
pub struct DockingPorts(pub u32);

//------------------------------------------------------------------------------
// PORTS
//------------------------------------------------------------------------------
pub enum PortSize {
    Outpost,
    Starport,
    Starhold,
    StarFortress,
    Citadel,
}

#[derive(PartialEq)]
pub enum PortModule {
    Food,
}

pub struct PortStats {
    pub name: String,
    pub modules: Vec<PortModule>,
    pub size: PortSize,
    pub scanner_range: u32,
    pub shields: u32,
    pub docking_ports: u32,
}

pub trait PortData {
    fn get_port_data(&self) -> PortStats;
}

pub struct FoodFactory;
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

//------------------------------------------------------------------------------
// FUNCTIONS
//------------------------------------------------------------------------------
pub fn create_port<T: PortData>(port_type: &T, world: &mut World, point: Option<Position>) {
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
        Name(port_stats.name),
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
