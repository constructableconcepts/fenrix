use fenrix_core::use_state;
use fenrix_dom::render;
use fenrix_macros::{component, rsx};
use wasm_bindgen::prelude::*;
use web_sys::Node;

#[allow(non_snake_case)]
#[component]
fn App() -> Node {
    // Create a signal to hold the text input.
    // `use_state` returns a `(getter, setter)` tuple.
    let text_signal = use_state(|| "Hello, Fenrix!".to_string());
    let text_getter = text_signal.0.clone();

    rsx! {
        <div>
            <h1>"Two-Way Data Binding Example"</h1>

            // The `bind:value` attribute creates a two-way binding.
            // It expands into an effect to set the input's value and
            // an `oninput` event listener to update the signal.
            <input type="text" bind:value={text_signal} />

            <hr />

            // This paragraph reactively displays the signal's current value.
            <p>"The current value is: "<b>{ text_getter() }</b></p>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    // Render the main `App` component to the DOM.
    render(rsx! { <App /> });
}