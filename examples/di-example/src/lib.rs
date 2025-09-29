use fenrix_core::{inject, provide_service};
use fenrix_dom::render;
use fenrix_macros::{component, rsx};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{console, MouseEvent, Node};

// 1. Define a concrete service.
// This service could be anything from an API client to a state manager.
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

// 2. Define a component that uses the service.
#[allow(non_snake_case)]
#[component]
fn App() -> Node {
    // 3. Inject the service.
    // The `inject` hook looks up the service in the global container by its type.
    let logger: Rc<LoggerService> = inject();

    let on_button_click = move |_: MouseEvent| {
        logger.log("Button clicked! Message from injected service.");
    };

    rsx! {
        <div>
            <h1>"Dependency Injection Example"</h1>
            <p>"Click the button to see a message in the console from the injected logger service."</p>
            <button (click)={on_button_click}>"Log Message"</button>
        </div>
    }
}

// 4. Set up the application entry point.
#[wasm_bindgen(start)]
pub fn run() {
    // 5. Provide the service instance to the container *before* any
    // component that needs it is rendered.
    provide_service(LoggerService::new("DI-LOGGER"));

    // 6. Render the root component.
    render(rsx! { <App /> });
}