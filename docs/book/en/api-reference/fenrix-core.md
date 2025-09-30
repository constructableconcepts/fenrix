# API Reference: `fenrix-core`

The `fenrix-core` crate provides the foundational building blocks for the Fenrix reactivity and dependency injection systems. This includes the hooks that your components will use to manage state and side effects.

---

## `use_state`

Creates a piece of reactive state that is tied to the lifecycle of the component it's called in. When the state changes, any part of the UI that depends on it will automatically update.

### Signature
```rust
pub fn use_state<T: Clone + 'static>(
    initial_value_fn: impl FnOnce() -> T
) -> (impl Fn() -> T + Clone, impl Fn(T) + Clone)
```

### Details
- **`initial_value_fn`**: A closure that returns the initial value of the state. This closure is only run the first time the component is rendered.
- **Returns**: A tuple containing a `(getter, setter)` pair.
    - The **getter** is a function that, when called, returns the current value of the state. Calling the getter inside an effect will subscribe that effect to the state's changes.
    - The **setter** is a function that takes a new value and updates the state.

### Example
```rust
#[component]
fn Counter() -> View {
    let count = use_state(|| 0);
    let count_getter = count.0.clone();
    let count_setter = count.1.clone();

    let increment = move |_| {
        count_setter(*count_getter() + 1);
    };

    rsx! {
        <div>
            <p>"Count: "{count_getter()}</p>
            <button on:click={increment}>"+"</button>
        </div>
    }
}
```

---

## `use_effect`

Creates a side effect that is tied to the component's lifecycle. The effect runs once after the component is first rendered, and will automatically re-run whenever any signals it depends on are updated.

### Signature
```rust
pub fn use_effect(effect_fn: impl Fn() + 'static)
```

### Details
- **`effect_fn`**: A closure that contains the side effect logic. If you read the value of a signal inside this closure, the effect will "subscribe" to that signal and re-run when the signal's value changes.

### Example
```rust
#[component]
fn Logger() -> View {
    let count = use_state(|| 0);

    use_effect({
        let count_getter = count.0.clone();
        move || {
            web_sys::console::log_1(&format!("The count is: {}", count_getter()).into());
        }
    });

    // ... (render logic for the counter)
}
```

---

## `provide_service`

Registers a service with the global dependency injection (DI) container. The service can then be accessed by any component using the `inject` hook.

### Signature
```rust
pub fn provide_service<T: 'static>(service: T)
```

### Details
- **`service`**: An instance of the struct or type you want to provide as a service.
- Services are stored by their type, so only one instance of any given type can be provided. This is typically called once at the application's entry point.
- It is common to wrap services in an `Rc<T>` to allow for shared ownership.

### Example
```rust
pub struct ApiClient { /* ... */ }

#[main]
pub fn main() {
    provide_service(Rc::new(ApiClient::new()));
    mount_to_body(App);
}
```

---

## `inject`

Retrieves a shared reference to a service from the global DI container.

### Signature
```rust
pub fn inject<T: 'static>() -> Rc<T>
```

### Details
- **Returns**: An `Rc<T>` pointing to the requested service instance.
- **Panics**: This function will panic if the requested service `T` has not been provided via `provide_service` before it is called.

### Example
```rust
#[component]
fn MyComponent() -> View {
    let api_client: Rc<ApiClient> = inject();

    // ... use api_client
}
```

---

## `create_signal` and `create_effect`

These are lower-level, standalone versions of `use_state` and `use_effect`. They are not tied to a component's lifecycle and can be used anywhere (e.g., in a global state management service).

- **`create_signal`**: Creates a new reactive signal.
- **`create_effect`**: Creates an effect that subscribes to signals.

Their usage is similar to their hook-based counterparts, but they offer more flexibility for advanced use cases outside of the component model.