use std::{net::UdpSocket, time::Duration};

use bevy::{app::ScheduleRunnerSettings, log::LogPlugin, prelude::*};
use bevy_simple_networking::{NetworkEvent, NetworkResource, ServerPlugin, Transport};

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

    App::build()
        // run the server at a reduced tick rate (100 ticks per minute)
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f32(
            60. / 100.,
        )))
        .insert_resource(socket)
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(ServerPlugin)
        .add_system(connection_handler.system())
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
            NetworkEvent::SendError(err, msg) => {
                info!(
                    "NetworkEvent::SendError (payload [{:?}]): {:?}",
                    msg.payload, err
                );
            }
            NetworkEvent::RecvError(err) => {
                info!("NetworkEvent::RecvError: {:?}", err);
            }
            _ => {}
        }
    }
}
