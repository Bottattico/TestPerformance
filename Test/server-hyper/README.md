## Getting Started
This simple project is inspired by an example from [wasmedge_hyper_demo](https://github.com/WasmEdge/wasmedge_hyper_demo/tree/main).

### 1. Build the Wasm Module

To begin, compile the Wasm module from the source code in this repository:

```sh
cargo component build --release
```
This command creates a Wasm component located at target/wasm32-wasi/release/wasmedge_hyper_server.wasm.

### 2. Install WasmEdge

You can find installation instructions on [wasmedge.org](https://wasmedge.org/).

### 3. (Optional) Compile with AOT
For ahead-of-time (AOT) compilation, use the following command:
```sh
wasmedge compile target/wasm32-wasi/release/wasmedge_hyper_server.wasm wasmedge_hyper_server_AOT.wasm
```
This will generate a compiled wasmedge_hyper_server_AOT.wasm file in your project directory, which is optimized for WasmEdge. For more information about AOT compilation, refer to the [AOT Compiler](https://wasmedge.org/docs/start/build-and-run/aot/).

### 4. Run the HTTP Server
To start the server, run one of the following commands:

- **Non-AOT Version**:

  ```sh
  wasmedge target/wasm32-wasi/release/wasmedge_hyper_server.wasm
  ```
- **AOT Version (if you compiled with AOT)**:

  ```sh
  wasmedge wasmedge_hyper_server_AOT.wasm  
  ```
By default, this will start an HTTP server listening on 0.0.0.0:8080.  

### 4. Make a Request
With the server running, open another terminal window and make a request:
```sh
curl http://localhost:8080
```

You should see the following response:

```sh
Hello
```
