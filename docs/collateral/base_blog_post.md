# Introducing Fenrix: A Faster, Safer Way to Build Full-Stack Web Apps with Rust

For years, the web has been dominated by JavaScript. But what if we could build web applications with the performance of C++ and the safety guarantees of a modern type system? What if we could eliminate entire classes of bugs at compile time?

This is the promise of Rust, and today, we're excited to introduce **Fenrix**, a new, full-stack web framework designed to bring Rust's power to the world of web development.

## Why Rust for the Web?

Rust is a modern programming language known for two game-changing features:

1.  **Blazing Speed:** Rust is compiled to native machine code, making it incredibly fast. For the web, this means faster server responses, quicker page loads, and smoother user experiences.
2.  **Fearless Concurrency & Safety:** Rust's unique ownership model and compiler enforce memory safety and prevent data races at compile time. This eliminates many of the most common and frustrating bugs that plague applications today.

Fenrix is built to leverage these strengths, providing a foundation for web applications that are not just fast, but also robust and reliable.

## What Makes Fenrix Different?

Fenrix is a complete toolkit for building modern web apps. It takes inspiration from the best ideas in popular frameworks like React and SolidJS and rebuilds them on a Rust foundation.

Here are the core features that make Fenrix special:

### 1. Fine-Grained Reactivity for Peak Performance

Many frameworks use a "Virtual DOM" (VDOM) to update the UI. This is a clever solution, but it involves extra work in the browser. Fenrix uses a **signal-based reactivity** model, which is far more efficient.

When a piece of data changes, Fenrix knows exactly which part of the UI needs to be updated and changes only that specific part. This results in surgical, high-performance updates and a snappier feel for the user.

```rust
// In a Fenrix component:
let (count, set_count) = use_state(0);

// This text updates automatically whenever `count` changes.
rsx! { <p>"Count: {count}"</p> }

// Clicking this button triggers the update.
rsx! { <button (click)={move |_| set_count(count() + 1)}>"+"</button> }
```

### 2. Isomorphic Server Functions: The End of the API

One of the most complex parts of web development is managing the client-server boundary. With Fenrix, this boundary virtually disappears thanks to **Isomorphic Server Functions**.

You can write a function that accesses your database and call it from your frontend component as if it were a local function. The framework handles the rest.

```mermaid
graph TD
    subgraph Browser (Client-side Component)
        A[UserProfile Component] --> B{ on_load: call get_user_from_db() };
        B --> C{Render user data};
    end

    subgraph Server (Backend Logic)
        D[#[server] async fn get_user_from_db] --> E[<i class='fa fa-database'></i> Access Database];
    end

    B -- Automatic Network Call --> D;
    style A fill:#D6EAF8
    style E fill:#FADBD8
```

Here’s what that looks like in code. You define a function and mark it as `#[server]`:

```rust
#[server]
async fn get_user_from_db(id: u32) -> Result<User, ServerFnError> {
    // This code ONLY runs on the server.
    // It's safe to use database connections or secrets here.
    let user = database::get_user(id).await?;
    Ok(user)
}
```

Then, you can call it directly from your frontend component:

```rust
// In a component that runs in the browser:
#[component]
fn UserProfile(id: u32) -> FenrixView {
    let user = use_resource(move || get_user_from_db(id));

    // Fenrix handles the loading state and makes the user data available.
    rsx! { <div>{user.name}</div> }
}
```

This feature drastically simplifies development, allowing you to focus on your application's logic, not on API boilerplate.

## Our First Release: Fenrix v0.0.1 (Alpha)

We are launching our first public alpha today. The core features are in place, and it's ready for enthusiasts and early adopters to start building and providing feedback.

If you're a developer interested in high-performance, type-safe web development, we invite you to join us.

- **Explore the code and examples:** [https://github.com/user/fenrix](https://github.com/user/fenrix)
- **Get started in minutes with our CLI:** `cargo install fenrix-cli && fenrix new my-app`

We believe that Rust is the future of high-performance web applications, and Fenrix is our first step in building that future. We can't wait to see what you create.