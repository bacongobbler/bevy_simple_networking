use std::net::{SocketAddr, UdpSocket};

use bevy::{log::LogPlugin, prelude::*};
use bevy_simple_networking::{ClientPlugin, NetworkEvent, NetworkSystem};

/// A marker component for our pnj.
/// Contains the unique ID of the png.
struct Pnj(u8);

struct ServerAddr(SocketAddr);

fn main() {
    let remote_addr: SocketAddr = "127.0.0.1:4567".parse().expect("could not parse addr");
    let socket = UdpSocket::bind("[::]:0").expect("could not bind socket");
    socket
        .connect(remote_addr)
        .expect("could not connect to server");
    socket
        .set_nonblocking(true)
        .expect("could not set socket to be nonblocking");

    App::new()
        .insert_resource(ServerAddr(remote_addr))
        .insert_resource(socket)
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(ClientPlugin)
        .add_startup_system(setup)
        .add_system(pnj_movement).before("send_pnjs_positions")
        .add_system(send_pnjs_positions.label("send_pnjs_positions").before(NetworkSystem::Send))
        .add_system(connection_handler)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn()
        .insert(Transform::from_xyz(0.0, 0.0, 0.0)
        .insert(Pnj(0));
    commands.spawn()
        .insert(Transform::from_xyz(3.0, 0.0, 0.0)
        .insert(Pnj(1));
    commands.spawn()
        .insert(Transform::from_xyz(0.0, 4.0, 0.0)
        .insert(Pnj(2));
}

fn pnj_movement(mut q: Query<&mut Transform, With<Pnj>>) {
    for transform in q.iter_mut() {
        transform.translation += Vec3::X;
    }
}

fn send_pnjs_positions(
    server_addr: Res<ServerAddr>,
    mut transport: ResMut<Transport>,
    pnjs: Query<(&Transform, &Pnj)>,
) {
    let server_addr = server_addr.0;
    for (transform, Pnj(id)) in pnjs.iter() {
        let message = Vec::with_capacity(13); // 1 + 4 + 4 + 4
        message.push(id);
        message.extend_from_slice(transport.translation.x.to_be_bytes());
        message.extend_from_slice(transport.translation.y.to_be_bytes());
        message.extend_from_slice(transport.translation.z.to_be_bytes());
        transport.send(server_addr, message);
    }
}

fn connection_handler(mut events: EventReader<NetworkEvent>) {
    for event in events.iter() {
        match event {
            NetworkEvent::Message(_, msg) => {
                info!("server sent a message: {:?}", msg);
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
