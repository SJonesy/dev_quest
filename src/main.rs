#![allow(unused)]

mod game;
mod server;

use crate::game::init;
use crate::server::run;
use bevy_ecs::prelude::*;
use dev_quest::{
    InternalOpcodeInstruction, PacketData, PlayerState, Players, ANSI, INTERNAL_OPCODE, TELNET,
};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, SystemTime};

fn main() -> std::io::Result<()> {
    let (send_to_main, read_from_server) = mpsc::channel();
    let (send_to_server, read_from_main) = mpsc::channel();
    thread::spawn(move || {
        server::run(send_to_main, read_from_main);
    });

    let mut global_player_state: Players = Players {
        state: [PlayerState::None; 4096],
    };

    let mut world = World::new();
    let mut schedule = Schedule::default();
    game::init(&mut world, &mut schedule);

    let mut time = SystemTime::now();

    // MAIN GAME LOOP
    loop {
        // HANDLE SERVER I/O
        match read_from_server.try_recv() {
            Ok(packet_data) => {
                println!("[Main] Receieved: {packet_data}");
                let token_num: usize = usize::from(packet_data.token);

                match *packet_data.data.first().unwrap() {
                    // TELNET CONTROL CODE
                    TELNET::IAC => {
                        // TODO: debug pong, remove eventually
                        send_to_server
                            .send(PacketData {
                                token: packet_data.token,
                                data: vec![0xFF, 1, 3, 3, 7],
                            })
                            .unwrap();
                    }
                    // INTERNAL CONTROL CODES
                    INTERNAL_OPCODE => {
                        let instruction =
                            InternalOpcodeInstruction::from(*packet_data.data.get(1).unwrap());
                        match instruction {
                            InternalOpcodeInstruction::SetPlayerState => {
                                let new_state =
                                    PlayerState::from(*packet_data.data.get(2).unwrap());
                                global_player_state.state[token_num] = new_state;
                                send_to_server
                                    .send(PacketData {
                                        token: packet_data.token,
                                        data: "Welcome to Dev Quest!".as_bytes().to_vec(),
                                    })
                                    .unwrap();
                            }
                            default => {}
                        }
                    }
                    default => {}
                }
            }
            Err(err) => {}
        };

        // DO GAME STUFF
        // doing 1 tick per second, I want to think about how this feels and what is optimal
        match time.elapsed() {
            Ok(elapsed) => {
                if (elapsed.as_millis() >= 1000) {
                    time = SystemTime::now();
                    game::tick(&mut world, &mut schedule);
                }
            }
            Err(e) => {
                println!("Error: {e:?}");
            }
        }
    }

    Ok(())
}
