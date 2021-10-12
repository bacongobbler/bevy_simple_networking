mod events;
mod message;
mod systems;
mod transport;
mod urgency;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;

pub use self::events::NetworkEvent;
pub use self::transport::Transport;

use bevy::prelude::*;

/// Defines how many times a client automatically sends a heartbeat packet.
/// This should be no more than half of idle_timeout.
const DEFAULT_HEARTBEAT_TICK_RATE_SECS: f32 = 2.;

pub struct NetworkResource {
    // Hashmap of each live connection and their last known packet activity
    pub connections: HashMap<SocketAddr, Duration>,
    pub idle_timeout: Duration,
}

impl Default for NetworkResource {
    fn default() -> Self {
        Self {
            connections: Default::default(),
            idle_timeout: Duration::from_secs(5),
        }
    }
}

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(NetworkResource::default())
            .insert_resource(transport::Transport::new())
            .add_event::<events::NetworkEvent>()
            .add_system(systems::server_recv_packet_system.system())
            .add_system(systems::send_packet_system.system())
            .add_system(systems::idle_timeout_system.system());
    }
}

pub struct HeartbeatTimer(Timer);

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(transport::Transport::new())
            .insert_resource(HeartbeatTimer(Timer::from_seconds(
                DEFAULT_HEARTBEAT_TICK_RATE_SECS,
                true,
            )))
            .add_event::<events::NetworkEvent>()
            .add_system(systems::client_recv_packet_system.system())
            .add_system(systems::send_packet_system.system())
            .add_system(systems::auto_heartbeat_system.system());
    }
}
