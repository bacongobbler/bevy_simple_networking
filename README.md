# Bevy Networking Plugin

![version](https://img.shields.io/crates/v/bevy_simple_networking)
![downloads](https://img.shields.io/crates/d/bevy_simple_networking)

This is a simple networking plugin for the Bevy game engine. This plugin
provides the building blocks which game developers can use to develop online
multiplayer games using the authoritative server model.

Currently, this plugin provides:

- An independent tick timer used for client/server synchronization
- UDP transport support
- message priority support (sent on the next available game tick or sent
  immediately)
- full connection life cycle management

By default, idle connections are dropped after 1 second, but this can be
configured by changing the value of a `NetworkResource`'s `idle_timeout`
parameter.

It is up to the developer to keep this connection alive. Usually this means by
sending a heartbeat packet on every tick. This can be done automatically if you
add the `auto_heartbeat_system` to your client.

## License

See [LICENSE](LICENSE).
