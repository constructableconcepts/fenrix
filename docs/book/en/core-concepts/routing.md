# Client-Side Routing

Fenrix includes a flexible, hash-based client-side router that allows you to build single-page applications (SPAs) with multiple views or pages. The router is provided as a service that can be injected into your components.

## How it Works

The Fenrix router uses the URL hash (the part of the URL after the `#`) to determine which component to display. For example, a URL like `http://localhost:8080/#/about` will be interpreted by the router to show the "About" page. This approach is simple and doesn't require any special server-side configuration.

## Setting up the Router

To get started with routing, you need to do three things:

1.  **Define your page components:** Each "page" in your application is a standard Fenrix component.
2.  **Create a route map:** You'll create a `HashMap` that maps URL paths (like `/` or `/about`) to your page components.
3.  **Provide the router:** At the entry point of your app, you'll initialize and provide the router service to the dependency injection container.

Here's how you would set it up in your `main` or `run` function:

```rust
use fenrix_router::{provide_router, Routable};
use std::collections::HashMap;

// 1. Define page components
#[component]
fn HomePage() -> Node { /* ... */ }

#[component]
fn AboutPage() -> Node { /* ... */ }

#[component]
fn NotFoundPage() -> Node {
    rsx! { <h1>"404: Page Not Found"</h1> }
}

// In your application's entry point:
#[wasm_bindgen(start)]
pub fn run() {
    // 2. Create the route map
    let mut routes = HashMap::new();
    routes.insert("/".to_string(), HomePage as Routable);
    routes.insert("/about".to_string(), AboutPage as Routable);

    // 3. Provide the router service
    provide_router(routes);

    // Render the main App component that will display the pages
    render(rsx! { <App /> });
}
```

## Displaying the Active Page

Your main `App` component is responsible for listening to URL changes and displaying the correct page component. This is typically done by:
1.  Injecting the router with `use_router()`.
2.  Using a signal to store the currently active page component.
3.  Creating an effect that listens for changes to the router's path and updates the signal.

```rust
#[component]
fn App() -> Node {
    let router = use_router();
    // A signal to hold the function pointer of the component to render.
    let (active_component, set_active_component) = use_state::<Routable>(|| NotFoundPage);

    // This effect runs whenever the URL hash changes.
    create_effect(move || {
        let path = (router.current_path)();
        let component_to_render = router.get_component(&path).unwrap_or(NotFoundPage);
        set_active_component(component_to_render);
    });

    rsx! {
        <div>
            <header>
                {/* Navigation links go here */}
            </header>
            <main>
                {/* Render the active component */}
                { ( (active_component())() ) }
            </main>
        </div>
    }
}
```

## Navigating with the `Link` Component

To allow users to navigate between pages, Fenrix provides a `Link` component. This component renders a standard `<a>` tag that, when clicked, updates the URL hash and triggers the router.

```rust
// Inside a component's rsx! macro:
rsx! {
    <nav>
        <Link to="/">"Home"</Link>
        <Link to="/about">"About"</Link>
    </nav>
}
```

-   The `to` prop specifies the path to navigate to (without the leading `#`).
-   The children of the `Link` component will be rendered inside the `<a>` tag.

Using this combination of services, hooks, and components, you can build complex, multi-page applications with Fenrix.