**Title:** Show HN: Fenrix, a full-stack Rust web framework with signal-based reactivity

**Body:**

I've been working on Fenrix, a web framework for building full-stack applications entirely in Rust, and I'm sharing the first alpha release.

My goal was to get the ergonomics of frameworks like React but with Rust's performance and without a VDOM. Fenrix uses a signal-based reactivity system (inspired by SolidJS/Leptos) for direct DOM updates.

The core feature is `#[server]` functions, which let you write server-side logic (e.g., DB queries) and call it directly from your client-side components. The framework abstracts the API layer.

Here is an example of a server function:
```rust
// This function only runs on the server
#[server]
async fn get_user(id: u32) -> Result<User, ServerFnError> {
    // ... safe to access a database or secrets here
}

// This component can be rendered on the client
#[component]
fn UserView(id: u32) -> FenrixView {
    let user_data = use_resource(move || get_user(id));

    // ... render user_data
}
```

It also includes a hook-based component model, an `rsx!` macro for templating, and a client-side router.

It's an early release, so I'm looking for feedback.

- GitHub: [https://github.com/user/fenrix](https://github.com/user/fenrix)
- Examples: (Link to the user guide/examples)

I'm here to answer any questions.