# Reactivity with Signals

At the heart of Fenrix is a powerful, fine-grained reactivity system based on **signals**. Signals are a way to manage state in your application that allows for efficient, surgical updates to the DOM, without the need for a virtual DOM (VDOM).

## What are Signals?

A signal is a reactive value. When a signal's value changes, any part of your application that depends on that signal will automatically update. This makes it easy to keep your UI in sync with your application's state.

In Fenrix, you can create a signal using the `use_state` hook:

```rust
use fenrix_core::use_state;

// ... inside a component
let count = use_state(|| 0);
```

The `use_state` hook returns a tuple containing a getter and a setter. To read the signal's value, you call the getter. To update the value, you call the setter.

```rust
// Get the current value
let current_count = count.0();

// Set a new value
count.1.set(current_count + 1);
```

## Two-Way Data Binding

Fenrix makes it easy to bind signals directly to form inputs, creating a two-way data binding. This means that any changes to the input will automatically update the signal, and any changes to the signal will automatically update the input's value.

You can create a two-way binding using the `bind:value` attribute in the `rsx!` macro:

```rust
use fenrix_core::use_state;
use fenrix_macros::{component, rsx};
use web_sys::Node;

#[allow(non_snake_case)]
#[component]
fn App() -> Node {
    let text_signal = use_state(|| "Hello, Fenrix!".to_string());
    let text_getter = text_signal.0.clone();

    rsx! {
        <div>
            <h1>"Two-Way Data Binding Example"</h1>

            // This input is bound to the `text_signal`
            <input type="text" bind:value={text_signal} />

            <hr />

            <p>"The current value is: "<b>{ text_getter() }</b></p>
        </div>
    }
}
```

In this example:
- The `<input>` element's value is bound to `text_signal`.
- When you type in the input field, the `text_signal` is automatically updated.
- The `<p>` tag reads the value from the signal via `text_getter()` and reactively displays it.

This declarative approach to state management simplifies your code and makes it easier to reason about how your application works. By using signals, you can build complex, interactive user interfaces with excellent performance.