## Let's start!
With that, build the Wasm component from source in this repository:
```sh
$ cargo component build --release
    Creating component target/wasm32-wasip1/release/hello_wasi_http.wasm
```

This builds a Wasm component at `target/wasm32-wasip1/release/hello_wasi_http.wasm`.

To run it, we'll need at least Wasmtime `18.0`. Installation instructions are on [wasmtime.dev].
[wasmtime.dev]: https://wasmtime.dev/

To compile it with AOT, you can use:
```sh
$  wasmtime compile target/wasm32-wasip1/release/hello_wasi_http.wasm
```
Then, in your project, will appear a magic hello_wasi_http.cwasm, in other world, a file compiled with AOT from Wasmtime.
For more info with AOT compiler look [cli-options].
[cli-options]: https://docs.wasmtime.dev/cli-options.html#compile

Then, run in your terminal:
```sh
$  wasmtime serve target/wasm32-wasip1/release/hello_wasi_http.wasm
```

Or, for the AOT version:
```sh
$  wasmtime serve hello_wasi_http.cwasm --allow-precompiled
```

This starts up an HTTP server that, by default, listens on `0.0.0.0:8080`.

With that running, in another window, we can now make requests!
```sh
$ curl http://localhost:8080
Hello!
```
