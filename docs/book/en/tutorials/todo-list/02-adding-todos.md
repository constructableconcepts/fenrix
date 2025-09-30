# 2. Adding New Todos

In this part of the tutorial, we'll bring our application to life by adding state management. This will allow users to type in new todos and see them added to the list.

## Defining the Todo State

First, we need a way to represent a single todo item. Let's define a `Todo` struct. It will hold the unique ID, text, and completion status of each todo. Add this at the top of your `src/lib.rs` file.

```rust
#[derive(Clone, PartialEq)]
struct Todo {
    id: u32,
    text: String,
    completed: bool,
}
```

We derive `Clone` so we can easily copy `Todo` items and `PartialEq` to allow comparing them.

## Managing State with Signals

To store our list of todos, we'll use a signal created with the `use_state` hook. This will make our UI automatically update whenever the list changes.

Let's modify the `App` component to hold the list of todos. We'll also need a way to keep track of the next available ID for new todos.

```rust
#[component]
fn App() -> View {
    // A signal to hold the list of todos
    let todos = use_state(Vec::<Todo>::new);
    // A signal for the next todo ID
    let next_id = use_state(|| 0);

    rsx! {
        <section class="todoapp">
            <header class="header">
                <h1>"todos"</h1>
                // We'll add the input component here
            </header>
            <TodoList todos={todos.0.clone()} />
        </section>
    }
}
```
Notice that we are now passing the `todos` signal getter to the `TodoList` component as a prop.

## Creating the `AddTodo` Input Component

Let's create a new component specifically for adding new todos. This component will contain the input field and the logic for creating a new todo when the user presses the "Enter" key.

```rust
#[component]
fn AddTodo(
    todos: Signal<Vec<Todo>>,
    next_id: Signal<u32>,
) -> View {
    let new_todo_text = use_state(String::new);

    let on_keydown = move |event: web_sys::KeyboardEvent| {
        if event.key() == "Enter" {
            let mut new_todos = (*todos.0()).clone();
            new_todos.push(Todo {
                id: *next_id.0(),
                text: new_todo_text.0().clone(),
                completed: false,
            });
            todos.1.set(new_todos);

            next_id.1.set(*next_id.0() + 1);
            new_todo_text.1.set(String::new());
        }
    };

    rsx! {
        <input
            class="new-todo"
            placeholder="What needs to be done?"
            bind:value={new_todo_text}
            on:keydown={on_keydown}
        />
    }
}
```
This component does a few things:
1.  It accepts the `todos` and `next_id` signals as props.
2.  It creates its own local state, `new_todo_text`, to hold the input's value.
3.  The `bind:value` directive creates a two-way binding between the input field and `new_todo_text`.
4.  The `on:keydown` event handler checks for the "Enter" key. When pressed, it creates a new `Todo`, adds it to the list, increments the ID, and clears the input field.

Now, let's add the `AddTodo` component to our `App`:
```rust
#[component]
fn App() -> View {
    let todos = use_state(Vec::<Todo>::new);
    let next_id = use_state(|| 0);

    rsx! {
        <section class="todoapp">
            <header class="header">
                <h1>"todos"</h1>
                <AddTodo todos={todos.clone()} next_id={next_id.clone()} />
            </header>
            <TodoList todos={todos.0.clone()} />
        </section>
    }
}
```

## Making the List Dynamic

Our `TodoList` and `TodoItem` components are still static. Let's make them dynamic by rendering the actual data from our `todos` signal.

First, we'll update the `TodoList` component. Instead of rendering a hardcoded list, it will now accept the `todos` signal's getter as a prop. Inside the `rsx!` macro, we'll call this getter to get the current list of todos, iterate over it, and render a `TodoItem` for each one. This ensures that whenever the `todos` signal changes, this component will re-render with the updated list.

```rust
#[component]
fn TodoList(todos: ReadSignal<Vec<Todo>>) -> View {
    rsx! {
        <section class="main">
            <ul class="todo-list">
                {
                    todos().into_iter().map(|todo| {
                        rsx! { <TodoItem todo={todo.clone()} /> }
                    }).collect::<Vec<_>>()
                }
            </ul>
        </section>
    }
}
```
We iterate over the `todos` vector and create a `TodoItem` for each one, passing the `todo` data as a prop.

Finally, update `TodoItem` to accept a `todo` prop and display its text.
```rust
#[component]
fn TodoItem(todo: Todo) -> View {
    rsx! {
        <li>
            <div class="view">
                <input class="toggle" type="checkbox" />
                <label>{&todo.text}</label>
                <button class="destroy"></button>
            </div>
        </li>
    }
}
```
Now, if you run `fenrix-cli dev`, you can type a new todo in the input field, press "Enter", and see it appear in the list!

In the next part, we'll add the logic for marking todos as complete.