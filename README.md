# Fenrix: A Modern Rust Web Framework

**Fenrix** is a full-fledged, Rust-centric web framework for building modern, performant, and dynamic web applications. Written entirely in Rust, it is designed to provide a highly ergonomic and productive development experience without sacrificing the raw performance and safety guarantees of the Rust language.

Inspired by the best ideas from established frameworks like Angular and React, and built upon the latest innovations in the Rust ecosystem, Fenrix aims to be the go-to choice for developers who want to build next-generation web apps with Rust.

## Core Features

- **Signal-Based Reactivity:** A fine-grained, VDOM-less reactivity model for state management (`use_state`). This ensures that UI updates are surgical and highly performant.
- **`rsx!` Templating:** A powerful macro for writing HTML-like syntax directly in your Rust code, with the full power of Rust expressions and compile-time safety.
- **Component Model:** A simple and powerful hook-based functional component system (`#[component]`, `use_effect`).
- **Isomorphic Server Functions:** The `#[server]` macro allows you to write server-side logic (like database queries) directly in your application code, abstracting away the client-server API layer.
- **Dependency Injection:** A built-in container for managing and injecting services and other dependencies into your components.
- **Client-Side Routing:** A simple but effective router for building single-page applications.
- **Developer Experience:** A dedicated command-line interface (`fenrix-cli`) for project creation and running a development server.

## Getting Started

To start building applications with Fenrix, you'll need the Rust toolchain and `wasm-pack` installed.

1.  **Install `wasm-pack`:**
    ```bash
    cargo install wasm-pack
    ```

2.  **Install the Fenrix CLI:**
    The CLI is included in this repository. You can install it directly:
    ```bash
    cargo install --path crates/fenrix-cli
    ```

3.  **Create a new project:**
    ```bash
    fenrix-cli new my-awesome-app
    ```

4.  **Run the development server:**
    ```bash
    cd my-awesome-app
    fenrix-cli dev
    ```
    Your application will be available at `http://127.0.0.1:8080`.

## Examples

This repository contains several examples in the `/examples` directory that demonstrate various features of Fenrix, from a simple "Hello, World!" to a full-stack server function example. They are a great way to learn about the framework's capabilities.

## Contributing

We welcome contributions! Please see our contributing guidelines for more information on how to get involved.

## License

Fenrix is licensed under the MIT License. See the `LICENSE` file for more details.