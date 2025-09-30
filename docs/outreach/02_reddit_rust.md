**Title:** I built a full-stack web framework in Rust, would love your feedback!

**Body:**

Hey /r/rust,

I've been working on a personal project for a while and wanted to share the first alpha: **Fenrix**, a full-stack web framework inspired by projects like Leptos and React. My goal was to create something that feels ergonomic for building modern web apps but is 100% Rust.

A few highlights:

*   **Signal-based reactivity:** No VDOM, just fast, fine-grained updates.
*   **Isomorphic `#[server]` functions:** This is the core idea. You can write a function that accesses a database or secrets and call it from your client-side component code as if it were a local async function. The framework handles the API boundary.
*   **Modern tooling:** It includes a hook-based component model (`use_state`, `use_effect`), an `rsx!` macro for templating, and a router.

Here's what server functions look like in practice:
```rust
#[server]
async fn get_user_name(user_id: u32) -> Result<String, ServerFnError> {
    // This code only ever runs on the server
    let user = db::get_user(user_id).await?;
    Ok(user.name)
}

#[component]
fn UserProfile(user_id: u32) -> FenrixView {
    // `use_resource` calls the server function and makes the result available
    let user_name = use_resource(move || get_user_name(user_id));

    rsx! { <p>"User: {user_name}"</p> }
}
```

It's still very early days, and I know there are rough edges. I have a huge amount of respect for the web dev work being done in this community and would be grateful for any feedback you have.

*   **GitHub (includes examples):** [https://github.com/user/fenrix](https://github.com/user/fenrix)
*   **User Guide:** (Link to the user guide)

I'll be around all day to answer questions. Thanks!