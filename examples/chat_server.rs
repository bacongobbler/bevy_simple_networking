use std::{net::UdpSocket, time::Duration};

use bevy::{app::ScheduleRunnerSettings, log::LogPlugin, prelude::*};
use bevy_simple_networking::{
    NetworkEvent, NetworkResource, ServerPlugin, Transport, UdpSocketResource,
};

const LISTEN_ADDRESS: &str = "127.0.0.1:4567";

fn main() {
    let socket = UdpSocket::bind(LISTEN_ADDRESS).expect("could not bind socket");
    socket
        .set_nonblocking(true)
        .expect("could not set socket to be nonblocking");
    socket
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("could not set read timeout");

    info!("Server now listening on {}", LISTEN_ADDRESS);

    App::new()
        // run the server at a reduced tick rate (35 ticks per second)
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f32(
            1. / 35.,
        )))
        .insert_resource(UdpSocketResource::new(socket))
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default())
        .add_plugin(ServerPlugin)
        .add_system(connection_handler)
        .run();
}

fn connection_handler(
    net: Res<NetworkResource>,
    mut events: EventReader<NetworkEvent>,
    mut transport: ResMut<Transport>,
) {
    for event in events.iter() {
        match event {
            NetworkEvent::Connected(handle) => {
                for (addr, _) in net.connections.iter() {
                    transport.send(*addr, format!("{} has entered the chat", handle).as_bytes());
                }
            }
            NetworkEvent::Disconnected(handle) => {
                for (addr, _) in net.connections.iter() {
                    transport.send(*addr, format!("{} has left the chat", handle).as_bytes());
                }
            }
            NetworkEvent::Message(handle, msg) => {
                for (addr, _) in net.connections.iter() {
                    transport.send(
                        *addr,
                        format!("{}: {}", handle, String::from_utf8_lossy(msg)).as_bytes(),
                    );
                }
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
        }
    }
}
