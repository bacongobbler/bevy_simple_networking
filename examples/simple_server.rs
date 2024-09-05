use std::{net::UdpSocket, time::Duration};

use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};
use bevy_simple_networking::{NetworkEvent, ServerPlugin, Transport, UdpSocketResource};

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
        .insert_resource(UdpSocketResource::new(socket))
        .add_plugins((
            // run the server at a reduced tick rate (100 ticks per minute)
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(60. / 100.))),
            LogPlugin::default(),
            ServerPlugin
        ))
        .add_systems(Update, connection_handler)
        .run();
}

fn connection_handler(mut events: EventReader<NetworkEvent>, mut transport: ResMut<Transport>) {
    for event in events.read() {
        match event {
            NetworkEvent::Connected(handle) => {
                info!("{}: connected!", handle);
                transport.send(*handle, b"PONG");
            }
            NetworkEvent::Disconnected(handle) => {
                info!("{}: disconnected!", handle);
            }
            NetworkEvent::Message(handle, msg) => {
                info!("{} sent a message: {:?}", handle, String::from_utf8_lossy(msg));
            }
            NetworkEvent::SendError(err, msg) => {
                error!(
                    "NetworkEvent::SendError (payload [{:?}]): {:?}",
                    String::from_utf8_lossy(&msg.payload), err
                );
            }
            NetworkEvent::RecvError(err) => {
                error!("NetworkEvent::RecvError: {:?}", err);
            }
        }
    }
}
