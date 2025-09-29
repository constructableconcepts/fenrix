use fenrix_dom::render;
use fenrix_macros::{component, rsx};
use wasm_bindgen::prelude::*;

/// A simple, static component.
#[allow(non_snake_case)]
#[component]
fn App() -> web_sys::Node {
    rsx! {
        <div>
            <h1>"Simple Component"</h1>
            <p>"This is rendered from a Fenrix component."</p>
        </div>
    }
}

/// The entry point for the application.
#[wasm_bindgen(start)]
pub fn run() {
    // Render the main `App` component to the DOM.
    // The `rsx!` macro now understands component tags.
    let app = rsx! { <App /> };
    render(app);
}