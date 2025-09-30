---
title: "I built a full-stack web framework in Rust. Here's how it works."
published: false
description: "Introducing Fenrix, a new Rust framework that uses server functions to make full-stack development feel seamless. Let's see how it works!"
tags: [rust, webdev, wasm, showcase]
---

## I got tired of writing APIs, so I built a web framework

I love Rust, and I love web development. But I've always found the boundary between the frontend and backend to be a bit... clunky. You write your backend logic, expose it via a REST or GraphQL API, then write a bunch of frontend code to fetch that data. It works, but it's a lot of boilerplate.

What if you could just... call your backend function directly from your frontend code?

That's the core idea that led me to build **Fenrix**, a new full-stack Rust web framework I just released in its first alpha.

### How does it work? Meet Server Functions

The magic feature in Fenrix is the `#[server]` macro. It lets you write a function that can access a database, use secret keys, or do any other server-only task, but you can call it from your client-side components as if it were a normal async function.

Here's a quick example. Imagine you want to fetch a user's name from your database. With Fenrix, you'd write this function:

```rust
// This function is defined in your shared app code,
// but it ONLY ever runs on the server.
#[server]
async fn get_user_name(user_id: u32) -> Result<String, ServerFnError> {
    // It's safe to talk to a database here!
    let user = database::get_user_by_id(user_id).await?;
    Ok(user.name)
}
```

Now, from a component that runs in the browser (WebAssembly), you can just call it:

```rust
// In your component code...
let name = get_user_name(123).await;

// `name` will contain the user's name fetched from the database!
// No manual `fetch` calls, no JSON serialization boilerplate.
```

The `#[server]` macro automatically creates an API endpoint and replaces the function call on the client with a network request. It makes building full-stack features incredibly fast.

### What about the frontend?

Fenrix isn't just about the backend. It's a complete framework with:

*   **Signal-based reactivity:** No VDOM! It's inspired by SolidJS, so it's super fast. When state changes, we update the DOM directly.
*   **An `rsx!` macro:** Write HTML-like templates right in your Rust code.
*   **A component system:** Functional components with hooks, similar to React.

### This is an alpha! I need your help.

Fenrix is brand new and there's still a lot to do. I'm looking for feedback from the community. If you're curious about a new take on web development in Rust, I'd be honored if you checked it out.

- **GitHub Repo & Examples:** [https://github.com/user/fenrix](https://github.com/user/fenrix)
- **Getting Started Guide:** (Link to the user guide)

What do you think of this approach? Let me know in the comments!