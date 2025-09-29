# Contributing to Fenrix

First off, thank you for considering contributing to Fenrix! It's people like you that make the open source community such a great place. We welcome any and all contributions.

This document provides guidelines for contributing to the project.

## Code of Conduct

This project and everyone participating in it is governed by the [Fenrix Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior.

## How Can I Contribute?

There are many ways to contribute, from writing tutorials or blog posts, improving the documentation, submitting bug reports and feature requests or writing code which can be incorporated into Fenrix itself.

### Reporting Bugs

- **Ensure the bug was not already reported** by searching on GitHub under [Issues](https://github.com/user/fenrix/issues).
- If you're unable to find an open issue addressing the problem, [open a new one](https://github.com/user/fenrix/issues/new). Be sure to include a **title and clear description**, as much relevant information as possible, and a **code sample** or an **executable test case** demonstrating the expected behavior that is not occurring.

### Suggesting Enhancements

- Open a new issue and provide a clear description of the enhancement you are suggesting.
- Explain why this enhancement would be useful to other Fenrix users.

## Your First Code Contribution

Unsure where to begin contributing to Fenrix? You can start by looking through `good first issue` and `help wanted` issues.

### Development Environment Setup

To get started with the Fenrix codebase, you'll need the following tools:

1.  **Rust Toolchain:** Fenrix is built with Rust. You can install the toolchain using `rustup`. If you already have it, make sure it's up to date (`rustup update`).
2.  **`wasm-pack`:** We use `wasm-pack` to build the WebAssembly modules for our examples and for testing. You can install it via `cargo`:
    ```sh
    cargo install wasm-pack
    ```

After cloning the repository, you can build the entire project to ensure everything is set up correctly:

```sh
cargo build --release
```

You can also run one of the examples:

```sh
cd examples/hello-world
wasm-pack build --target web
```

### Pull Request Process

1.  Fork the repository and create your branch from `main`.
2.  If you've added code that should be tested, add tests.
3.  Ensure the test suite passes (`cargo test --all-targets`).
4.  Format your code with `cargo fmt`.
5.  If you've changed APIs, update the documentation.
6.  Make sure your code lints (`cargo clippy --all-targets -- -D warnings`).
7.  Issue that pull request!

### Commit Message and Branch Naming Conventions

- **Branch Names:** Please use kebab-case for branch names (e.g., `feat/add-new-component`).
- **Commit Messages:** We follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification. This allows for automated changelog generation. Each commit message consists of a **header**, a **body** and a **footer**.
    - The header is mandatory and has a special format that includes a **type**, a **scope** and a **subject**:
      ```
      <type>(<scope>): <subject>
      ```
      Example: `feat(compiler): add support for svg elements`

Thank you for your contribution!