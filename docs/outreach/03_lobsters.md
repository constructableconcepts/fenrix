**Title:** Fenrix: A Rust web framework with signal-based reactivity and isomorphic server functions

**Tags:** rust, web, wasm, show

I've released the first alpha of Fenrix, a full-stack Rust web framework.

The design is based on two main ideas:

1.  **Signal-based reactivity without a VDOM.** The `rsx!` macro compiles templates into efficient DOM creation code. State changes are propagated through signals, which trigger direct, fine-grained DOM updates. This approach is inspired by SolidJS and Leptos.

2.  **Isomorphic server functions via a `#[server]` macro.** This macro splits a function into a server-only implementation and a client-side stub that makes a network request. This removes the need for a manually-written API layer and allows data-fetching logic to be co-located with components.

The framework also provides a hook-based component model and a client-side router.

I'm looking for feedback on these architectural choices.

- **Repo:** [https://github.com/user/fenrix](https://github.com/user/fenrix)
- **Technical Article:** (Link to the technical article)

I'm available to discuss the design and answer questions.