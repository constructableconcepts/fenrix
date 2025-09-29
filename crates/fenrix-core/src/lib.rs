use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// The internal state of a signal.
struct Signal<T> {
    value: T,
    subscribers: Vec<Rc<dyn Fn()>>,
}

type AnySignal = Rc<dyn Any>;

// The context for a single component instance, managing its hooks.
#[derive(Default)]
struct ComponentContext {
    states: Vec<AnySignal>,
    effects: Vec<()>, // Use a unit type to simply mark that an effect has been created.
    state_index: usize,
    effect_index: usize,
}

// A thread-local stack to manage nested component renders.
thread_local! {
    static CONTEXT_STACK: RefCell<Vec<ComponentContext>> = RefCell::new(Vec::new());
    static CURRENT_EFFECT: RefCell<Option<Rc<dyn Fn()>>> = RefCell::new(None);
}

/// Provides a piece of state for a component.
pub fn use_state<T: Clone + 'static>(
    initial_value_fn: impl FnOnce() -> T,
) -> (impl Fn() -> T + Clone, impl Fn(T) + Clone) {
    CONTEXT_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        let current_context = stack
            .last_mut()
            .expect("`use_state` can only be called inside a component.");

        if let Some(any_signal) = current_context.states.get(current_context.state_index) {
            current_context.state_index += 1;
            let signal = any_signal
                .clone()
                .downcast::<RefCell<Signal<T>>>()
                .expect("Mismatched state type in `use_state` hook.");
            return create_signal_from_rc(signal);
        }

        let initial_value = initial_value_fn();
        let signal = Rc::new(RefCell::new(Signal {
            value: initial_value,
            subscribers: Vec::new(),
        }));

        current_context.states.push(signal.clone() as AnySignal);
        current_context.state_index += 1;
        create_signal_from_rc(signal)
    })
}

/// A reactive signal that holds a value.
pub fn create_signal<T: Clone + 'static>(
    initial_value: T,
) -> (impl Fn() -> T + Clone, impl Fn(T) + Clone) {
    let signal = Rc::new(RefCell::new(Signal {
        value: initial_value,
        subscribers: Vec::new(),
    }));
    create_signal_from_rc(signal)
}

// Helper to create getter/setter from a signal Rc.
fn create_signal_from_rc<T: Clone + 'static>(
    signal: Rc<RefCell<Signal<T>>>,
) -> (impl Fn() -> T + Clone, impl Fn(T) + Clone) {
    let getter = {
        let signal = Rc::clone(&signal);
        move || {
            CURRENT_EFFECT.with(|e| {
                if let Some(effect) = e.borrow().clone() {
                    let mut s = signal.borrow_mut();
                    if !s.subscribers.iter().any(|s_rc| Rc::ptr_eq(s_rc, &effect)) {
                        s.subscribers.push(effect);
                    }
                }
            });
            signal.borrow().value.clone()
        }
    };

    let setter = {
        let signal = Rc::clone(&signal);
        move |new_value: T| {
            let subscribers = {
                let mut s = signal.borrow_mut();
                s.value = new_value;
                s.subscribers.clone()
            };
            for effect in subscribers {
                effect();
            }
        }
    };

    (getter, setter)
}

/// Creates an effect that runs once and re-runs when its dependencies change.
pub fn use_effect(effect_fn: impl Fn() + 'static) {
    CONTEXT_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        let current_context = stack
            .last_mut()
            .expect("`use_effect` can only be called inside a component.");

        if current_context.effects.get(current_context.effect_index).is_some() {
            current_context.effect_index += 1;
            return;
        }

        create_effect(effect_fn);
        current_context.effects.push(());
        current_context.effect_index += 1;
    })
}

/// Creates an effect that re-runs when its dependencies change.
pub fn create_effect(effect_fn: impl Fn() + 'static) {
    let effect = Rc::new(move || {
        effect_fn();
    });

    CURRENT_EFFECT.with(|e| e.borrow_mut().replace(effect.clone()));
    effect();
    CURRENT_EFFECT.with(|e| e.borrow_mut().take());
}

/// A helper function to be called by the `#[component]` macro.
pub fn with_component_context<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    CONTEXT_STACK.with(|s| s.borrow_mut().push(ComponentContext::default()));
    let result = f();
    CONTEXT_STACK.with(|s| s.borrow_mut().pop());
    result
}

// A container for dependency-injected services.
#[derive(Default)]
pub struct ServiceContainer {
    services: HashMap<TypeId, Rc<dyn Any>>,
}

impl ServiceContainer {
    pub fn new() -> Self {
        Self::default()
    }
}

thread_local! {
    // A thread-local static container for services.
    static SERVICE_CONTAINER: RefCell<ServiceContainer> = RefCell::new(ServiceContainer::new());
}

/// Provides a service to the application's global DI container.
///
/// The service is stored by its type, so only one instance of any
/// given type can be provided.
pub fn provide_service<T: 'static>(service: T) {
    SERVICE_CONTAINER.with(|sc| {
        sc.borrow_mut()
            .services
            .insert(TypeId::of::<T>(), Rc::new(service));
    });
}

/// Injects a service from the global DI container.
///
/// This hook retrieves a shared reference (`Rc`) to a service that was
/// previously provided via `provide_service`.
///
/// # Panics
///
/// This function will panic if the requested service (`T`) has not been
/// provided, or if the stored service cannot be downcast to the requested type.
pub fn inject<T: 'static>() -> Rc<T> {
    SERVICE_CONTAINER.with(|sc| {
        let sc = sc.borrow();
        let service = sc
            .services
            .get(&TypeId::of::<T>())
            .expect("Service not provided: Make sure to call `provide_service` before `inject`.");

        service
            .clone()
            .downcast::<T>()
            .expect("Failed to downcast service to the requested type.")
    })
}