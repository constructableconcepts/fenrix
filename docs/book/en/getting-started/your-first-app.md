# Your First App

Now that you have the Fenrix CLI installed, you're ready to create your first application. In this guide, we'll build a simple "Hello, World!" app to introduce you to the basic concepts of Fenrix.

## Create a New Project

To create a new project, use the `fenrix-cli new` command:

```bash
fenrix-cli new my-fenrix-app
```

This will create a new directory called `my-fenrix-app` with the following structure:

```
my-fenrix-app/
├── Cargo.toml
├── index.html
└── src/
    └── lib.rs
```

-   `Cargo.toml`: The standard Rust package manifest, with `fenrix` added as a dependency.
-   `index.html`: The main HTML file for your application.
-   `src/lib.rs`: The entry point for your Fenrix application.

## Run the Development Server

To see your app in action, navigate to the project directory and run the development server:

```bash
cd my-fenrix-app
fenrix-cli dev
```

The CLI will compile your application and start a development server. You can view your app by opening `http://127.0.0.1:8080` in your web browser.

## Understanding the Code

Let's take a look at the code in `src/lib.rs`:

```rust
use fenrix::{component, render, rsx, web_sys::Node};
use wasm_bindgen::prelude::*;

#[allow(non_snake_case)]
#[component]
fn App() -> Node {
    rsx! {
        <div>
            <h1>"Hello, Fenrix!"</h1>
            <p>"Your new Fenrix application is running."</p>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    render(rsx! { <App /> });
}
```

-   **`#[component]`**: This attribute marks the `App` function as a Fenrix component. Components are the building blocks of a Fenrix application.
-   **`rsx!`**: This macro allows you to write HTML-like syntax directly in your Rust code. You can embed Rust expressions directly in the template using curly braces `{}`.
-   **`#[wasm_bindgen(start)]`**: This attribute marks the `run` function as the entry point for your WebAssembly module.
-   **`render(rsx! { <App /> })`**: This function takes the `App` component, renders it, and mounts it to the `<body>` of the HTML document.

Congratulations, you've just built your first Fenrix application! In the next section, we'll explore the core concepts of Fenrix in more detail.