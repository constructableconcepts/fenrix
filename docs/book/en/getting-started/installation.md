# Installation

Before you can start building applications with Fenrix, you'll need to set up your development environment. This guide will walk you through the process of installing the necessary tools.

## Install Rust

If you don't already have Rust installed, you can install it using `rustup`, the official Rust toolchain installer.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This command will download and run `rustup-init`, which will install the latest stable version of Rust. If you already have `rustup` installed, you can update your toolchain by running:

```bash
rustup update
```

## Install `wasm-pack`

`wasm-pack` is a tool for building and packaging Rust-generated WebAssembly. You can install it using `cargo`, the Rust package manager:

```bash
cargo install wasm-pack
```

## Install the Fenrix CLI

To install it, you will need to have the Fenrix source code on your local machine. Once you have the code, you can install the CLI from the `crates/fenrix-cli` directory within the project:

```bash
# Navigate to the root of the Fenrix project directory
# Then, run the install command:
cargo install --path crates/fenrix-cli
```

Once the installation is complete, you can verify that the CLI is working by running:

```bash
fenrix-cli --version
```

With these tools installed, you're ready to create your first Fenrix application!