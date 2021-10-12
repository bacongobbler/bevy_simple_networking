use std::{net::UdpSocket, time::Duration};

use bevy::prelude::*;
use bevy_simple_networking::{NetworkEvent, ServerPlugin};

const LISTEN_ADDRESS: &str = "127.0.0.1:4567";

fn main() {
    let socket = UdpSocket::bind(LISTEN_ADDRESS).expect("could not bind socket");
    socket
        .set_nonblocking(true)
        .expect("could not set socket to be nonblocking");
    socket
        .set_broadcast(true)
        .expect("could not set SO_BROADCAST");
    socket
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("could not set read timeout");

    println!("Server now listening on {}", LISTEN_ADDRESS);

    App::build()
        .insert_resource(socket)
        .add_plugins(MinimalPlugins)
        .add_plugin(ServerPlugin)
        .add_system(connection_handler.system())
        .run();
}

pub fn connection_handler(mut events: EventReader<NetworkEvent>) {
    for event in events.iter() {
        match event {
            NetworkEvent::Connected(handle) => {
                println!("{}: connected!", handle);
            }
            NetworkEvent::Disconnected(handle) => {
                println!("{}: disconnected!", handle);
            }
            NetworkEvent::Message(handle, msg) => {
                println!("{} sent a message: {:?}", handle, msg);
            }
            NetworkEvent::SendError(err, msg) => {
                println!(
                    "NetworkEvent::SendError (payload [{:?}]): {:?}",
                    msg.payload, err
                );
            }
            NetworkEvent::RecvError(err) => {
                println!("NetworkEvent::RecvError: {:?}", err);
            }
            NetworkEvent::ConnectionError(err, handle) => match handle {
                Some(h) => println!("NetworkEvent::ConnectionError from {}: {:?}", h, err),
                _ => println!("NetworkEvent::ConnectionError: {:?}", err),
            },
        }
    }
}
