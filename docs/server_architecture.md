# Fenrix Server Architecture

This document outlines the architecture for handling server-side logic, specifically functions annotated with the `#[server]` macro, within the Fenrix framework.

## Core Concept: Dedicated Server Binaries

The fundamental principle of Fenrix's server-side functionality is that **any application or example that includes server-side logic must have its own dedicated server binary.**

A generic, pre-compiled tool like `fenrix-cli` cannot be used to serve applications that rely on `#[server]` functions.

### Rationale

The `#[server]` macro works by splitting a single function definition into two distinct implementations:

1.  **Client-side (Wasm):** The function body is replaced with a `fetch` call to a corresponding API endpoint (e.g., `/api/my_function_name`).
2.  **Server-side (Native):** The original function body, containing the actual server logic (e.g., database queries, file system access), is kept.

For the server-side logic to be executable, it must be compiled into a native binary that can be run on a server. A pre-compiled `fenrix-cli` binary is compiled independently and has no knowledge of the specific server functions defined in a user's application crate. Therefore, it cannot execute that logic.

Each part of the application with server logic must compile its own server.

## How It Works: The `server-function-example` Model

The `server-function-example` serves as the template for this architecture.

### 1. Crate Structure

The example is structured as a Rust crate with both a library and a binary target:

-   **`src/lib.rs`**: Contains the shared code, including the UI components and the `#[server]` function definitions. This library is compiled to WebAssembly for the client.
-   **`src/main.rs`**: This is the entry point for the dedicated server binary. It is only compiled for native targets.
-   **`Cargo.toml`**: Defines both the `lib` and `bin` targets. It uses Cargo features to separate server-only dependencies (like `tokio` and `fenrix-server`) from the shared codebase, preventing them from being included in the Wasm build.

### 2. The Server Binary (`main.rs`)

The role of the server binary is to:

1.  **Collect Server Functions:** It gathers all the `#[server]` functions defined in the library.
2.  **Create a Dispatch Map:** It creates a `HashMap` where keys are the string names of the server functions and values are type-erased function pointers to their actual implementations.
3.  **Configure the Server:** It creates a `ServerConfig` struct provided by the `fenrix-server` crate. This config includes the server's address, the path to the static client assets, and the `HashMap` of server functions.
4.  **Start the Server:** It calls the `fenrix_server::start_server()` function, passing it the configuration.

### 3. The `fenrix-server` Crate

This crate provides a reusable, `axum`-based web server with the following responsibilities:

-   Serving static files (the client-side Wasm/JS/HTML) from a specified directory.
-   Providing a dynamic API route (e.g., `/api/:name`).
-   When a request hits the API route, the `handle_api` function looks up the function name in the `HashMap` it received in its configuration, executes the corresponding function with the request's payload, and returns the result.

### 4. The `run_system_test.sh` Script

For examples with server functions, the test script implements special handling:

-   It first builds the client-side application using `wasm-pack`.
-   It then builds the dedicated server binary using `cargo build --bin <server_name>`.
-   It runs the compiled server binary in the background.
-   It then proceeds with the Playwright verification against the running server.
-   For purely client-side examples, it continues to use the simpler `fenrix-cli dev` command.