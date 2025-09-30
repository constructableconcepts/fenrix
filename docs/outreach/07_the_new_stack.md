**Headline:** New Rust Framework Fenrix Unifies Full-Stack Development with Server Functions

**Body:**

A new open-source web framework, Fenrix, has entered its alpha release, offering a novel approach to full-stack development in Rust. The project's headline feature, isomorphic server functions, aims to eliminate the traditional API layer between client and server, allowing developers to call backend logic directly from frontend components.

Fenrix enters a growing ecosystem of Rust-based tools targeting WebAssembly, where performance and safety are key drivers. Unlike many popular frameworks that rely on a Virtual DOM (VDOM), Fenrix uses a signal-based, VDOM-less rendering architecture. This model, popularized by frameworks like SolidJS, is designed to minimize runtime overhead by updating only the specific DOM nodes that have changed, avoiding the need to diff an entire component tree.

The framework's most distinct feature is its implementation of server functions. By annotating a standard Rust function with a `#[server]` macro, a developer can write code that accesses server-only resources like databases or secret keys. The Fenrix compiler then intelligently splits the function: the body is compiled exclusively for the server, while the call site on the client is replaced with a transparent network request to an auto-generated API endpoint.

According to the project's authors, this design is intended to drastically reduce the boilerplate associated with creating and managing REST or GraphQL APIs, a common bottleneck in modern web development.

The v0.0.1 alpha release also includes a suite of tools familiar to web developers, such as a hook-based component model, an `rsx!` macro for templating, and a client-side router.

As organizations continue to adopt WebAssembly for performance-critical applications, frameworks like Fenrix—which prioritize both raw speed and developer ergonomics—could signal a maturing of the Rust web ecosystem.

The project is available on GitHub and is currently seeking feedback from early adopters.