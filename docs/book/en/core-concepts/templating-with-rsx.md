# Templating with rsx!

Fenrix uses the `rsx!` macro to provide a powerful and ergonomic way to write your application's UI. The `rsx!` macro allows you to write HTML-like syntax directly in your Rust code, with the added benefits of compile-time checking and the full power of Rust expressions.

## Basic Syntax

The `rsx!` macro looks very similar to HTML:

```rust
rsx! {
    <div>
        <h1>"Hello, World!"</h1>
        <p>"This is a Fenrix application."</p>
    </div>
}
```

You can create any standard HTML elements, and the macro will transform them into the necessary code to create and manage the corresponding DOM nodes.

## Embedding Rust Expressions

One of the most powerful features of `rsx!` is the ability to embed Rust expressions directly into your templates. You can do this by wrapping the expression in curly braces `{}`.

```rust
let name = "Fenrix";
let message = rsx! { <p>{"Hello, "}{name}{"!"}</p> };
```

This is not limited to simple variables. You can use any valid Rust expression, including function calls, arithmetic operations, and more.

```rust
let a = 10;
let b = 20;
let sum_message = rsx! { <p>{"The sum of a and b is: "}{a + b}</p> };
```

## Using Components

The `rsx!` macro is also how you render components. When the macro encounters a tag that starts with a capital letter, it treats it as a component.

```rust
#[component]
fn Greeting() -> web_sys::Node {
    rsx! { <h1>"Hello from the Greeting component!"</h1> }
}

// ... later in another component
let view = rsx! {
    <div>
        <p>"Here is a custom component:"</p>
        <Greeting />
    </div>
};
```

This allows you to build your UI by composing components in a declarative and readable way.

## Attributes and Event Handlers

You can set attributes on elements just like in HTML. String literals are used for static values, and expressions in curly braces `{}` can be used for dynamic values.

```rust
let class_name = "my-class";
let is_disabled = true;

let element = rsx! {
    <div class="container" id={class_name}>
        <button disabled={is_disabled}>"Click me"</button>
    </div>
};
```

Event handlers can be attached using the `on:` syntax, followed by the event name (e.g., `onclick`, `oninput`).

```rust
use fenrix_core::use_state;

let count = use_state(|| 0);
let count_setter = count.1.clone();

let button = rsx! {
    <button onclick={move |_| count_setter.set(*count.0() + 1)}>
        "Click me"
    </button>
};
```

The `rsx!` macro is a central piece of the Fenrix development experience, providing a safe, powerful, and intuitive way to build user interfaces.