# Server Functions

Fenrix provides a powerful feature called **server functions** that allows you to write server-side code directly within your client-side application logic. This simplifies the process of creating full-stack applications by abstracting away the need to manually create and manage API endpoints.

## What are Server Functions?

A server function is a Rust function that you can call from your client-side code (i.e., in your components), but which only runs on the server. This is ideal for tasks like database queries, authentication, or any other operation that requires a secure server environment.

You can create a server function by marking an `async` function with the `#[server]` attribute.

## How it Works

When you use the `#[server]` macro, Fenrix does two things:

1.  **On the server:** It generates a standard API endpoint that wraps your function.
2.  **On the client:** It replaces the body of your function with a call to that API endpoint.

This means you can write your code as if you are calling a local function, and Fenrix handles all the client-server communication for you.

## Example

Here's an example of a server function that simulates fetching a user from a database:

```rust
use fenrix_macros::{component, rsx, server};
use serde::{Deserialize, Serialize};

// Data structures must be serializable
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    id: u32,
    name: String,
}

// This function is defined once, but has two different implementations
#[server]
async fn get_user_from_db(id: u32) -> Result<User, String> {
    // This is the server-side implementation.
    // It will only be included in the server binary.
    println!("Server: Received request for user with id: {}", id);
    if id == 1 {
        Ok(User {
            id: 1,
            name: "GV".to_string(),
        })
    } else {
        Err("User not found".to_string())
    }
}

#[component]
pub fn App() -> web_sys::Node {
    // ... component logic ...

    // Calling the server function from the client
    wasm_bindgen_futures::spawn_local(async move {
        match get_user_from_db(1).await {
            Ok(fetched_user) => {
                // We got the user from the server!
                // Now, update the UI.
            }
            Err(e) => {
                // Handle the error
            }
        }
    });

    // ... rsx! template ...
}
```

### Key Requirements

1.  **`async` functions:** Server functions must be `async`.
2.  **Serializable Types:** The arguments and return types of a server function must be serializable and deserializable using `serde`. This is because the data needs to be sent over the network.
3.  **Calling from the Client:** Since the client-side version of the function performs a network request, it is `async`. You should call it from within an async block, for example, using `wasm_bindgen_futures::spawn_local` in an event handler.

Server functions are a key feature for building full-stack applications with Fenrix, providing a seamless and ergonomic way to bridge the gap between your client and server code.