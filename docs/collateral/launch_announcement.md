# Announcing Fenrix v0.0.1: A Rust Web Framework for a New Generation

Today, we are thrilled to announce the initial public alpha release of **Fenrix v0.0.1**, a web framework designed to make Rust a first-class citizen for building powerful, modern, and full-stack web applications.

For too long, web development has been fragmented. We write frontend code, then backend code, and spend countless hours building and maintaining an API to bridge the two. Fenrix challenges this paradigm with a simple question: **What if you could just write your app?**

Our vision is to provide a truly seamless development experience that combines Rust's unparalleled performance and safety with the ergonomics you expect from a modern framework.

### The End of the API: Isomorphic Server Functions

The cornerstone of Fenrix is **Isomorphic Server Functions**. This powerful feature allows you to write server-side logic directly inside your application code and call it from your client-side components as if it were a local function. The framework handles the entire network boundary for you.

```mermaid
graph TD
    subgraph Browser (WebAssembly)
        A[Component: UserProfile] --> B{await get_user(123)};
    end

    subgraph Server (Rust)
        D[Function: get_user] --> E[<i class='fa fa-database'></i> Database];
    end

    B -- Transparent network call --> D;

    style A fill:#D6EAF8
    style E fill:#FADBD8
```

### A Complete Toolkit for Modern Web Apps

This initial release is more than just a concept; it's a comprehensive set of tools to build real applications:

- **Fine-Grained Reactivity:** A signal-based system for lightning-fast UI updates without a Virtual DOM.
- **Type-Safe `rsx!` Macro:** Write HTML-like templates directly in your Rust code with full confidence.
- **Modern Component Model:** A familiar, hook-based system for managing state and side effects.
- **Built-in Essentials:** Dependency injection and client-side routing included out of the box.
- **A Dedicated CLI:** Get up and running in minutes.

### Get Started with Fenrix Today

We invite you to explore Fenrix, try out the examples, and see what you can build. This is an early alpha, and your feedback will be invaluable as we build towards a stable release.

**Install the CLI and create your first app:**
```bash
cargo install fenrix-cli
fenrix new my-app
```

**Explore the Project:**
- **GitHub Repository:** [https://github.com/user/fenrix](https://github.com/user/fenrix)
- **v0.0.1 Release Notes:** [View the full release notes](./release_notes_v0.0.1.md)

Join us in building the next generation of web applications. We can't wait to see what you create.