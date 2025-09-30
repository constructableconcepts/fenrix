**Title:** How to build a full-stack Rust app with Fenrix's server functions

## Introduction

Rust's expansion into web development via WebAssembly (Wasm) offers incredible performance and safety. However, creating a full-stack application often involves significant boilerplate to manage the client-server divide. What if we could make this process seamless?

In this tutorial, we’ll explore Fenrix, a new Rust web framework designed to radically simplify full-stack development. We will build a simple "user fetcher" application from scratch, demonstrating how Fenrix's two core features—signal-based reactivity and isomorphic server functions—work together to create a powerful and ergonomic developer experience.

### Prerequisites
- A working Rust environment and `wasm-pack`.
- The Fenrix CLI installed (`cargo install fenrix-cli`).

## Setting up our Fenrix project

First, let's create a new Fenrix project using the CLI:
```bash
fenrix new fenrix-logrocket-app
cd fenrix-logrocket-app
```
This command scaffolds a new project with all the necessary files. We'll be doing most of our work in `src/lib.rs`.

## Defining our data and server logic

Our goal is to click a button on the frontend to fetch user data from the server.

First, let's define a simple `User` struct in `src/lib.rs`.
```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    id: u32,
    name: String,
    email: String,
}
```
The `Serialize` and `Deserialize` traits from `serde` are crucial here, as they allow this struct to be passed across the network.

Next, let's create the function to fetch a user. This is where Fenrix shines. Instead of setting up a REST endpoint, we just write a Rust function and annotate it with the `#[server]` macro.

Add this function to `lib.rs`:
```rust
#[server]
async fn get_user_from_db(id: u32) -> Result<User, String> {
    // This code is guaranteed to only run on the server.
    // We can safely access databases or secrets here.
    // Let's simulate a database lookup.
    println!("Server: Received request for user with id: {}", id);

    if id == 1 {
        Ok(User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        })
    } else {
        Err("User not found".to_string())
    }
}
```
The `#[server]` macro instructs Fenrix to treat this as a server-only function. The client-side Wasm bundle will contain a stub that makes a network call to this function, but the function body itself will never be exposed to the client.

## Building the frontend component

Now, let's build the UI component that will call our server function. Replace the default `App` component in `src/lib.rs` with the following code:

```rust
use fenrix_core::use_state;
use fenrix_macros::{component, rsx};
use wasm_bindgen::prelude::*;
use web_sys::Node;

// ... (keep the User struct and get_user_from_db function from before)

#[component]
pub fn App() -> Node {
    // 1. Create reactive state (signals) to hold our data.
    let (user, set_user) = use_state(None::<User>);
    let (is_loading, set_is_loading) = use_state(false);

    // 2. Define the event handler for our button click.
    let handle_click = {
        let set_user = set_user.clone();
        let set_is_loading = set_is_loading.clone();
        move |_: web_sys::MouseEvent| {
            let set_user = set_user.clone();
            let set_is_loading = set_is_loading.clone();
            
            // 3. Call our async server function from the client.
            wasm_bindgen_futures::spawn_local(async move {
                set_is_loading(true);
                match get_user_from_db(1).await {
                    Ok(fetched_user) => set_user(Some(fetched_user)),
                    Err(e) => web_sys::console::error_1(&e.into()),
                }
                set_is_loading(false);
            });
        }
    };

    // 4. Define the UI with the rsx! macro.
    rsx! {
        <div>
            <h1>"User Data Fetcher"</h1>
            <button (click)={handle_click} disabled={*is_loading}>
                { if *is_loading { "Loading..." } else { "Fetch User Data" } }
            </button>
            
            <div style="margin-top: 20px; padding: 10px; border: 1px solid #ccc;">
                {
                    // This UI is reactive. It will update whenever the `user` signal changes.
                    if let Some(user_data) = user() {
                        rsx! {
                            <div>
                                <h3>"User Found:"</h3>
                                <p><strong>"ID: "</strong>{user_data.id}</p>
                                <p><strong>"Name: "</strong>{user_data.name}</p>
                                <p><strong>"Email: "</strong>{user_data.email}</p>
                            </div>
                        }
                    } else {
                        rsx! { <p>"Click the button to fetch user data."</p> }
                    }
                }
            </div>
        </div>
    }
}
```

### How the component works

1.  **Reactive State:** We use Fenrix's `use_state` hook to create signals. Signals are reactive values; when they change, any part of the UI that depends on them will automatically update.
2.  **Event Handling:** We define a standard `onclick` handler for our button.
3.  **Calling the Server:** Inside our handler, we use `wasm_bindgen_futures::spawn_local` to run our `async` code. We simply `await` the `get_user_from_db` function as if it were a local function. Fenrix handles the network request, serialization, and deserialization behind the scenes.
4.  **Reactive UI:** The `rsx!` macro lets us write HTML-like syntax. The UI automatically re-renders based on the current state of our `user` and `is_loading` signals.

## Running the application

To see our app in action, run the Fenrix development server:
```bash
fenrix dev
```
Open your browser to `http://127.0.0.1:8080`. You should see our "User Data Fetcher" UI. When you click the button, the app will display a loading state, and then the user data will appear.

If you check the terminal where you ran the `fenrix dev` command, you will see the log message: "Server: Received request for user with id: 1". This confirms that our server function executed on the server, not in the browser.

## Conclusion

Fenrix's approach to full-stack development with Rust offers a significant improvement in developer ergonomics. By abstracting away the boilerplate of a traditional API layer, its server functions allow developers to focus more on business logic and less on plumbing. Combined with a high-performance, signal-based rendering model, Fenrix presents a compelling vision for the future of web development in Rust. This alpha release is a promising foundation for building fast, safe, and seamless web applications.