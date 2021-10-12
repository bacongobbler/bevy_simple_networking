mod events;
mod message;
mod systems;
mod time;
mod transport;
mod urgency;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;

pub use self::events::NetworkEvent;
pub use self::systems::auto_heartbeat_system;
pub use self::time::NetworkTime;
pub use self::transport::Transport;

use bevy::prelude::*;

pub struct NetworkResource {
    // Hashmap of each live connection and their last known packet activity
    pub connections: HashMap<SocketAddr, Duration>,
    pub idle_timeout: Duration,
}

impl Default for NetworkResource {
    fn default() -> Self {
        Self {
            connections: Default::default(),
            idle_timeout: Duration::from_secs(1),
        }
    }
}

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(NetworkResource::default())
            .insert_resource(time::NetworkTime::default())
            .insert_resource(transport::Transport::new())
            .add_event::<events::NetworkEvent>()
            .add_system(systems::network_time_system.system())
            .add_system(systems::network_recv_system.system())
            .add_system(systems::network_send_system.system())
            .add_system(systems::idle_timeout_system.system());
    }
}
