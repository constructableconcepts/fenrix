use fenrix_core::{create_effect, use_state};
use fenrix_dom::render;
use fenrix_macros::{component, rsx};
use fenrix_router::{provide_router, use_router, Routable};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::Node;

#[allow(non_snake_case)]
#[component]
fn HomePage() -> Node {
    rsx! { <div>"Welcome to the Home Page!"</div> }
}

#[allow(non_snake_case)]
#[component]
fn AboutPage() -> Node {
    rsx! { <div>"This is the About Page."</div> }
}

#[allow(non_snake_case)]
#[component]
fn NotFoundPage() -> Node {
    rsx! { <div>"404: Page Not Found"</div> }
}

#[allow(non_snake_case)]
#[component]
fn App() -> Node {
    let router = use_router();
    let (active_component, set_active_component) = use_state::<Routable>(|| NotFoundPage);

    // This effect listens to the router's `current_path` signal.
    // When the path changes, it finds the corresponding component
    // and updates the `active_component` signal, causing a re-render.
    let router_clone = router.clone();
    create_effect(move || {
        let path = (router_clone.current_path)();
        let component = router_clone.get_component(&path).unwrap_or(NotFoundPage);
        set_active_component(component);
    });

    rsx! {
        <div>
            <header>
                <h1>"Fenrix Router Example"</h1>
                <nav>
                    <Link to="/">"Home"</Link>
                    <span style="margin: 0 8px;">"|"</span>
                    <Link to="/about">"About"</Link>
                    <span style="margin: 0 8px;">"|"</span>
                    <Link to="/invalid-path">"Invalid Link"</Link>
                </nav>
            </header>
            <hr />
            <main>
                // This expression calls the active component function to render it.
                // The outer parentheses are a convention to tell the macro this
                // expression evaluates to a Node and should not be formatted as text.
                { ( (active_component())() ) }
            </main>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    // 1. Define the application's routes.
    let mut routes = HashMap::new();
    routes.insert("/".to_string(), HomePage as Routable);
    routes.insert("/about".to_string(), AboutPage as Routable);

    // 2. Provide the router service to the DI container.
    provide_router(routes);

    // 3. Render the main App component.
    render(rsx! { <App /> });
}