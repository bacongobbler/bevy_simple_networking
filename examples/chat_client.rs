use std::net::{SocketAddr, UdpSocket};

use bevy::{log::LogPlugin, prelude::*};
use bevy_simple_networking::{
    ClientPlugin, NetworkEvent, SocketAddrResource, Transport, UdpSocketResource,
};

fn main() {
    let remote_addr: SocketAddr = "127.0.0.1:4567".parse().expect("could not parse addr");
    let socket = UdpSocket::bind("0.0.0.0:0").expect("could not bind socket");
    socket
        .set_nonblocking(true)
        .expect("could not set socket to be nonblocking");

    App::new()
        .insert_resource(SocketAddrResource::new(remote_addr))
        .insert_resource(UdpSocketResource::new(socket))
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default())
        .add_plugin(ClientPlugin)
        .add_system(connection_handler)
        .add_startup_system(hello_world)
        .run();
}

fn connection_handler(mut events: EventReader<NetworkEvent>) {
    for event in events.iter() {
        match event {
            NetworkEvent::Message(_, msg) => {
                info!("{}", String::from_utf8_lossy(msg));
            }
            NetworkEvent::SendError(err, msg) => {
                error!(
                    "NetworkEvent::SendError (payload [{:?}]): {:?}",
                    msg.payload, err
                );
            }
            NetworkEvent::RecvError(err) => {
                error!("NetworkEvent::RecvError: {:?}", err);
            }
            // discard irrelevant events
            _ => {}
        }
    }
}

fn hello_world(remote_addr: Res<SocketAddrResource>, mut transport: ResMut<Transport>) {
    transport.send(**remote_addr, b"Hello world!");
}
