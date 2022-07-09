# wasi tcp server

Adapted from https://github.com/second-state/wasmedge_wasi_socket/blob/main/examples/tcp_listener.rs with annotations.

![image](https://user-images.githubusercontent.com/54848194/178113431-40a5d5ee-0908-4ff2-b5fb-709d4ed077f2.png)

The following command compiles the Rust program.

```
$ cargo build --target wasm32-wasi --release
```

Install WasmEdge
```
$ curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
$ source $HOME/.wasmedge/env
```

The following command runs the application in WasmEdge.

```
$ wasmedge target/wasm32-wasi/release/wasi-tcp-server.wasm
listening at 127.0.0.1:1234
```

Use netcat to connect
```
$ nc localhost 1234
```

