Example of mutual deadlock during tcp data transfer when both sides try to send large message.

```
cargo build
```

In one terminal:
```
./target/debug/server
```

In another terminal:
```
./target/debug/client
```
