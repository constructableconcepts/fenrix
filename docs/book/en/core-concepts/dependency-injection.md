# Dependency Injection

Fenrix includes a simple but powerful dependency injection (DI) system that allows you to provide services (like an API client, a state manager, or a logger) to your components without tightly coupling them. This makes your application more modular, easier to test, and more maintainable.

## The DI System

The DI system in Fenrix is based on two main functions:

-   `provide_service(service)`: This function registers an instance of a service with a global container. Any component rendered after this call can access the service.
-   `inject::<T>()`: This is a hook that components can use to look up and receive an instance of a service of type `T` from the container.

## How to Use DI

Let's walk through an example of creating and using a simple logging service.

### 1. Define Your Service

First, define the service you want to share. A service is just a Rust struct.

```rust
use web_sys::console;

pub struct LoggerService {
    prefix: String,
}

impl LoggerService {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
        }
    }

    pub fn log(&self, message: &str) {
        console::log_1(&format!("[{}] {}", self.prefix, message).into());
    }
}
```

### 2. Provide the Service

Before you render any components that need the service, you must provide an instance of it to the DI container. This is typically done at the entry point of your application.

```rust
use fenrix_core::provide_service;
use std::rc::Rc;

// ... at the start of your main function
provide_service(Rc::new(LoggerService::new("MY-APP")));
```

**Note on `Rc<T>`:** Services are often shared between multiple components. To manage this shared ownership, it's common practice to wrap your service in a reference-counted pointer like `Rc<T>` (for single-threaded contexts like WebAssembly) or `Arc<T>` (for multi-threaded contexts).

### 3. Inject the Service into a Component

Now, any component in your application can get access to the `LoggerService` by using the `inject` hook.

```rust
use fenrix_core::inject;
use fenrix_macros::{component, rsx};
use std::rc::Rc;
use web_sys::{MouseEvent, Node};

// ... (LoggerService definition)

#[allow(non_snake_case)]
#[component]
fn MyComponent() -> Node {
    // Inject the service by its type.
    // The type must match what was provided.
    let logger: Rc<LoggerService> = inject();

    let on_button_click = move |_: MouseEvent| {
        logger.log("Button was clicked!");
    };

    rsx! {
        <button onclick={on_button_click}>"Log a Message"</button>
    }
}
```

When `inject::<Rc<LoggerService>>()` is called, the DI container looks for a service of that exact type and returns it. If the service is not found, your application will panic, so it's important to ensure that services are provided before they are needed.

By using this pattern, `MyComponent` doesn't need to know how to create a `LoggerService`. It just declares that it needs one, and the DI system provides it. This decoupling is a powerful tool for building clean, well-structured applications.