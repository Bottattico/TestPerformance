## Getting Started

### 1. Build the Wasm Component

To begin, compile the Wasm component from the source code in this repository:

```sh
cargo component build --release
```
This command creates a Wasm component located at target/wasm32-wasip1/release/hello_wasi_http.wasm.

### 2. Install Wasmtime

To run this component, you'll need at least version 18.0 of Wasmtime. You can find installation instructions on [wasmtime.dev](https://wasmtime.dev/).

### 3. (Optional) Compile with AOT
For ahead-of-time (AOT) compilation, use the following command:
```sh
wasmtime compile target/wasm32-wasip1/release/hello_wasi_http.wasm
```
This will generate a precompiled hello_wasi_http.cwasm file in your project directory, which is optimized for Wasmtime. For more information about AOT compilation, refer to the [CLI options documentation](https://docs.wasmtime.dev/cli-options.html#compile).

### 4. Run the HTTP Server
To start the server, run one of the following commands:

- **Non-AOT Version**:

  ```sh
  wasmtime serve target/wasm32-wasip1/release/hello_wasi_http.wasm
  ```
- **AOT Version (if you compiled with AOT)**:

  ```sh
  wasmtime serve hello_wasi_http.cwasm --allow-precompiled  
  ```
By default, this will start an HTTP server listening on 0.0.0.0:8080.  

### 4. Make a Request
With the server running, open another terminal window and make a request:
```sh
curl http://localhost:8080
```

You should see the following response:

```sh
Hello!
```
