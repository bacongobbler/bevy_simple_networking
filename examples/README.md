# Client/Server Example

This provides an example client/server architecture. The server listens for new
connections on port 4567. Any clients spawned will automatically send heartbeat
packets to the server. The server logs any clients that connected or
disconnected by failing to send a heartbeat packet. Upon receiving the first
packet, the server responds with a "PONG".

In one terminal, run:

```
cd bevy_simple_networking
cargo run --example simple_server
```

In another terminal, run:

```
cd bevy_simple_networking
cargo run --example simple_client
```

You should see a message that the client connected in the server logs:

```
$ cargo run --example simple_server
   Compiling bevy_simple_networking v0.1.2 (/home/bacongobbler/code/bevy_simple_networking)
    Finished dev [unoptimized + debuginfo] target(s) in 8.54s
     Running `target/debug/examples/simple_server`
Oct 12 15:17:01.024  INFO simple_server: 127.0.0.1:37810: connected!
```

And you should see a response from the server in the client logs:

```
$ cargo run --example simple_client
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/simple_client`
Oct 12 16:16:17.097  INFO simple_client: server sent a message: b"PONG"
```

Now close the client by pressing CTRL+C (CMD+C on MacOS). After a few seconds,
you should see the following in the server logs:

```
 $ cargo run --example simple_server
   Compiling bevy_simple_networking v0.1.2 (/home/bacongobbler/code/bevy_simple_networking)
    Finished dev [unoptimized + debuginfo] target(s) in 8.54s
     Running `target/debug/examples/simple_server`
Oct 12 15:17:01.024  INFO simple_server: 127.0.0.1:37810: connected!
Oct 12 15:17:07.024  INFO simple_server: 127.0.0.1:37810: disconnected!
```
