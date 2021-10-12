use std::net::{SocketAddr, UdpSocket};

use bevy::{log::LogPlugin, prelude::*};
use bevy_simple_networking::{ClientPlugin, NetworkEvent};

fn main() {
    let remote_addr: SocketAddr = "127.0.0.1:4567".parse().expect("could not parse addr");
    let socket = UdpSocket::bind("[::]:0").expect("could not bind socket");
    socket
        .connect(remote_addr)
        .expect("could not connect to server");
    socket
        .set_nonblocking(true)
        .expect("could not set socket to be nonblocking");

    App::build()
        .insert_resource(remote_addr)
        .insert_resource(socket)
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(ClientPlugin)
        .add_system(connection_handler.system())
        .run();
}

fn connection_handler(mut events: EventReader<NetworkEvent>) {
    for event in events.iter() {
        match event {
            NetworkEvent::Message(_, msg) => {
                info!("server sent a message: {:?}", msg);
            },
            NetworkEvent::SendError(err, msg) => {
                error!(
                    "NetworkEvent::SendError (payload [{:?}]): {:?}",
                    msg.payload, err
                );
            },
            NetworkEvent::RecvError(err) => {
                error!("NetworkEvent::RecvError: {:?}", err);
            },
            // discard irrelevant events
            _ => {},
        }
    }
}
