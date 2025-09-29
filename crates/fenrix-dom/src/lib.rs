use fenrix_core::create_effect;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{window, Document, Element, Node, Text};

/// Gets the `document` object from the browser.
fn document() -> Document {
    window()
        .expect("should have a window")
        .document()
        .expect("window should have a document")
}

/// Creates a new HTML element with the given tag name.
pub fn create_element(tag: &str) -> Element {
    document()
        .create_element(tag)
        .expect("failed to create element")
}

/// Creates a new text node with the given content.
pub fn create_text_node(text: &str) -> Text {
    document().create_text_node(text)
}

/// Appends a child node to a parent element.
pub fn append_child(parent: &Element, child: &Node) {
    parent
        .append_child(child)
        .expect("failed to append child");
}

/// Creates a text node that reactively updates when its source changes.
///
/// # Arguments
///
/// * `source`: A closure that returns the string to be displayed. This closure
///   is tracked by the reactive system.
pub fn create_reactive_text_node(source: impl FnMut() -> String + 'static) -> Text {
    // Wrap the source closure in Rc<RefCell<>> to allow for shared mutable access
    // from within the `Fn` closure required by `create_effect`.
    let source = Rc::new(RefCell::new(source));

    // Get the initial value by calling the closure.
    let initial_value = source.borrow_mut()();
    let text_node = create_text_node(&initial_value);

    // Create an effect that updates the node's text content whenever the source changes.
    let node_clone = text_node.clone();
    create_effect({
        let source = Rc::clone(&source);
        move || {
            // When the effect runs, borrow the closure again to get the new value.
            let new_value = source.borrow_mut()();
            node_clone.set_node_value(Some(&new_value));
        }
    });

    text_node
}

/// Renders a root node to the document body.
pub fn render(root_node: Node) {
    let body = document().body().expect("document should have a body");
    body.append_child(&root_node)
        .expect("failed to append to body");
}