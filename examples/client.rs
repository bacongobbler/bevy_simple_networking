use std::net::{SocketAddr, UdpSocket};

use bevy::prelude::*;
use bevy_simple_networking::ClientPlugin;

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
        .add_plugin(ClientPlugin)
        .run();
}
