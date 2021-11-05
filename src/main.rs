#![allow(unused)]

mod server;

use std::sync::mpsc;
use std::thread;

use crate::server::run;
use dev_quest::PacketData;


fn main() -> std::io::Result<()> {
    let (send_to_main, read_from_server) = mpsc::channel();
    let (send_to_server, read_from_main) = mpsc::channel();
    thread::spawn(move || {
        server::run(send_to_main, read_from_main);
    });

    // MAIN GAME LOOP
    loop {
        // HANDLE SERVER I/O
        match read_from_server.recv() {
            Ok(packet_data) => { 
                println!("[Main] Receieved: {}", packet_data);
                if *packet_data.data.get(0).unwrap() == b'\xFF' {
                    send_to_server.send(PacketData {
                        token: packet_data.token,
                        data: vec![0xFF, 1, 3, 3, 7]
                    }).unwrap();
                };
            }
            Err(err) => { }
        };

        // DO GAME STUFF
    }

    Ok(())
}
