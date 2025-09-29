//! # Fenrix
//!
//! A modern Rust UI framework for building fast, reliable, and beautiful web apps.
//!
//! This crate is the main entry point for the Fenrix framework, re-exporting all the necessary
//! components from the various internal crates.

// Re-export core reactivity and component model
pub use fenrix_core::{
    create_effect, create_signal, inject, provide_service, use_effect, use_state, with_component_context,
};

// Re-export DOM rendering
pub use fenrix_dom::render;

// Re-export procedural macros
pub use fenrix_macros::{component, rsx};

// Re-export router components
pub use fenrix_router::{provide_router, use_router, Routable, Router};

// Re-export common dependencies for convenience
pub use wasm_bindgen;
pub use web_sys;