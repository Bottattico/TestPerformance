# Project Overview

This repository contains two different server implementations, one based on the `wasmtime` runtime and the other using the `wasmedge` runtime with the `hyper` crate.

## Prerequisites

Before running the projects, make sure you have the following tools installed:

- **[Wasmtime](https://wasmtime.dev/)**: The runtime for running WebAssembly outside the browser.
  - Installation instructions can be found on the [Wasmtime website](https://wasmtime.dev/).

- **[WasmEdge](https://wasmedge.org/)**: A high-performance WebAssembly runtime optimized for edge computing.
  - Installation instructions can be found on the [WasmEdge website](https://wasmedge.org/).

- **Cargo Component**: A cargo subcommand for building WebAssembly components.
  - You can install it using the following command:

    ```sh
    cargo install cargo-component
    ```
    
## Folder Structure

### **`hello-wasi-http-main`**
This folder contains the server implementation originally created by Sunfishcode using the `wasmtime` runtime. The server is built to run a simple WASI HTTP example, demonstrating how to serve HTTP requests in WebAssembly using `wasmtime`.

- **Purpose**: This is the original `hello_wasi_http` server running in a `wasmtime` environment.
- **Key Files**:
  - `Cargo.toml`: The configuration file for building the project.
  - `src/`: Contains the source code for the `wasmtime` server.

### **`server-hyper`**
This folder contains a modified version of the `wasmedge_hyper_demo`, which is a demonstration of the `hyper` crate running in a WebAssembly environment with the `wasmedge` runtime.

- **Purpose**: This is a custom implementation of a WebAssembly server using `wasmedge` and the `hyper` crate for HTTP request handling.
- **Key Files**:
  - `Cargo.toml`: The configuration file for building the project.
  - `src/`: Contains the source code for the `wasmedge` server with `hyper` integration.

## Differences Between the Servers

- **`hello-wasi-http-main`**: A basic WebAssembly HTTP server example running in `wasmtime`, as originally created by Sunfishcode.
- **`server-hyper`**: A more advanced WebAssembly HTTP server using `wasmedge` with custom modifications and the `hyper` crate for handling HTTP requests.

Both implementations showcase different approaches to running WebAssembly-based servers, using different runtimes and HTTP handling libraries.
