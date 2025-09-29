use fenrix_core::{use_effect, use_state};
use fenrix_dom::render;
use fenrix_macros::{component, rsx};
use wasm_bindgen::prelude::*;
use web_sys::{console, MouseEvent, Node};

/// A stateful counter component that uses declarative events.
#[allow(non_snake_case)]
#[component]
fn Counter() -> Node {
    let (count, set_count) = use_state(|| 0);
    let count_for_effect = count.clone();
    let count_for_click = count.clone();

    use_effect(move || {
        console::log_1(&format!("The count is now: {}", count_for_effect()).into());
    });

    // The event handler is now a simple closure passed directly to the `(click)` attribute.
    // The `rsx!` macro handles all the event listener boilerplate.
    rsx! {
        <div id="main">
            <h1>"Stateful Counter with Declarative Events"</h1>
            <p>"Current count: " {count()}</p>
            <button (click)={move |_: MouseEvent| set_count(count_for_click() + 1)}>
                "Increment"
            </button>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    render(rsx! { <Counter /> });
}