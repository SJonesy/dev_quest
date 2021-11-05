use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::collections::{hash_map, HashMap};
use std::io::{Read, Write};
use std::sync::mpsc::{self, TryRecvError};
use std::time::Duration;

use dev_quest::PacketData;


pub fn run(
    out_channel: mpsc::Sender<PacketData>,
    in_channel: mpsc::Receiver<PacketData>,
) -> std::io::Result<()> {
    const LISTEN_SOCKET: Token = Token(0);

    let address = "0.0.0.0:1337".parse().unwrap();
    let mut listener = TcpListener::bind(address).expect("Failed to bind");

    let mut poll = Poll::new().unwrap();
    poll.registry()
        .register(&mut listener, LISTEN_SOCKET, Interest::READABLE)?;

    let mut counter: usize = 0;
    let mut connections: HashMap<Token, TcpStream> = HashMap::new();
    let mut incoming_packets: Vec<PacketData> = Vec::new();
    let mut outgoing_packets: HashMap<Token, Vec<PacketData>> = HashMap::new();
    let mut buffer = [0 as u8; 65535];
    let mut events = Events::with_capacity(1024);

    // SERVER I/O LOOP
    loop {
        // READ DATA FROM MAIN
        match in_channel.try_recv() {
            Ok(packet_data) => {
                let token = packet_data.token;
                match outgoing_packets.entry(token) {
                    hash_map::Entry::Vacant(e) => {
                        e.insert(vec![packet_data]);
                    }
                    hash_map::Entry::Occupied(mut e) => {
                        e.get_mut().push(packet_data);
                    }
                };
                poll.registry().reregister(
                    connections.get_mut(&token).unwrap(),
                    token,
                    Interest::WRITABLE
                ).expect("Reregister failed.");
            }
            Err(err) => {
                if err == TryRecvError::Disconnected {
                    // TODO: can we recover from this?
                    panic!("Pipeline Main->Server has disconnected.");
                }
            }
        }
        // READ/WRITE FROM/TO SOCKETS
        poll.poll(&mut events, Some(Duration::from_millis(1)))
            .expect("Polling error");
        for event in &events {
            println!("Event: {:?}", event);
            match event.token() {
                LISTEN_SOCKET => {
                    // Accept new connections
                    match listener.accept() {
                        Ok((mut stream, addr)) => {
                            counter += 1;
                            let token = Token(counter);
                            poll.registry()
                                .register(&mut stream, token, Interest::READABLE)
                                .expect("Register failed.");
                            connections.insert(token, stream);
                            println!("Accepted connection from: {}", addr);
                        }
                        Err(e) => {
                            println!("Accept error from: {:?}", e);
                        }
                    }
                }
                token if event.is_readable() => {
                    let connection = connections.get_mut(&token).unwrap();
                    let read_result = connection.read(&mut buffer);
                    match read_result {
                        Ok(0) => {
                            println!("Disconnection: {:?}", connection.peer_addr());
                            connections.remove(&token);
                        }
                        Ok(num_bytes) => {
                            println!("Receieved {} bytes from {:?}", num_bytes, connection.peer_addr());
                            incoming_packets.push(PacketData {
                                token,
                                data: buffer[..num_bytes].to_vec(),
                            });
                        }
                        Err(e) => {
                            println!("Socket read error: {:?}", e);
                        }
                    }
                }
                token if event.is_writable() && outgoing_packets.contains_key(&token) => {
                    for packet_data in outgoing_packets.remove(&token).unwrap() {
                        connections
                            .get_mut(&token)
                            .unwrap()
                            .write(packet_data.data.as_slice())
                            .expect("Write failed.");
                    }
                    poll.registry().reregister(
                        connections.get_mut(&token).unwrap(),
                        token,
                        Interest::READABLE,
                    ).expect("Reregister failed.");
                }
                _ => unreachable!(),
            }
        }
        // WRITE DATA TO MAIN
        for packet_data in incoming_packets.drain(..) {
            out_channel.send(packet_data)
                .expect("Error sending data to main");
        }
    }
}