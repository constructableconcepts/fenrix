use fenrix_core::{create_signal, provide_service, inject};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HashChangeEvent, Node};

// A component function that returns a renderable node.
pub type Routable = fn() -> Node;

// The Router service that manages routes and the current path.
#[derive(Clone)]
pub struct Router {
    routes: Rc<HashMap<String, Routable>>,
    // The current path is a reactive signal.
    pub current_path: Rc<dyn Fn() -> String>,
    set_current_path: Rc<dyn Fn(String)>,
}

impl Router {
    pub fn new(routes: HashMap<String, Routable>) -> Self {
        let (current_path, set_current_path) = create_signal(get_current_hash());

        let router = Self {
            routes: Rc::new(routes),
            current_path: Rc::new(current_path),
            set_current_path: Rc::new(set_current_path),
        };

        // Listen for hash changes to update the current_path signal.
        let router_clone = router.clone();
        let on_hash_change = Closure::wrap(Box::new(move |_: HashChangeEvent| {
            (router_clone.set_current_path)(get_current_hash());
        }) as Box<dyn FnMut(_)>);

        window()
            .unwrap()
            .add_event_listener_with_callback("hashchange", on_hash_change.as_ref().unchecked_ref())
            .unwrap();
        on_hash_change.forget();

        router
    }

    // Returns the component function for the given path, or None if not found.
    pub fn get_component(&self, path: &str) -> Option<Routable> {
        self.routes.get(path).cloned()
    }
}

// Helper to get the current URL hash, defaulting to "/".
fn get_current_hash() -> String {
    let hash = window().unwrap().location().hash().unwrap_or_else(|_| "".to_string());
    if hash.is_empty() {
        "/".to_string()
    } else {
        // Remove the leading '#'
        hash[1..].to_string()
    }
}

/// Provides a router instance to the application.
pub fn provide_router(routes: HashMap<String, Routable>) {
    provide_service(Router::new(routes));
}

/// A hook to get the current router instance from the DI container.
pub fn use_router() -> Rc<Router> {
    inject::<Router>()
}