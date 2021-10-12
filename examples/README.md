# Client/Server Example

This provides an example client/server architecture. The server listens for new
connections on port 4567. Any clients spawned will automatically send heartbeat
packets to the server. The server logs any clients that connected or
disconnected by failing to send a heartbeat packet.

In one terminal, run:

```
cd bevy_simple_networking
cargo run --example server
```

In another terminal, run:

```
cd bevy_simple_networking
cargo run --example client
```

You should see the following in the server logs:

```
$ cargo run --example server
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/server`
Server now listening on 127.0.0.1:4567
127.0.0.1:35005: connected!
```

Now close the client by pressing CTRL+C (CMD+C on MacOS). You should see the
following in the server logs:

```
 $ cargo run --example server
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/server`
Server now listening on 127.0.0.1:4567
127.0.0.1:35005: connected!
127.0.0.1:35005: disconnected!
```
