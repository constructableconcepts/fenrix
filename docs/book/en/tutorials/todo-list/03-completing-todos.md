# 3. Completing Todos

Now that we can add todos, the next step is to allow users to mark them as complete. This will involve handling events on the checkbox in our `TodoItem` component and updating the state accordingly.

## Passing State Down

Currently, our `TodoItem` component only knows about a single `Todo`'s data. It doesn't have a way to communicate changes back up to the `App` component where the list of todos is stored. To fix this, we need to pass the main `todos` signal down to each `TodoItem`.

First, let's update the `TodoList` component to pass the full `todos` signal to each `TodoItem` it renders.

```rust
#[component]
fn TodoList(todos: Signal<Vec<Todo>>) -> View {
    rsx! {
        <section class="main">
            <ul class="todo-list">
                {
                    // Note: We are iterating over the getter part of the signal
                    todos.0().into_iter().map(|todo| {
                        rsx! { <TodoItem todo={todo.clone()} todos={todos.clone()} /> }
                    }).collect::<Vec<_>>()
                }
            </ul>
        </section>
    }
}
```
And update the `App` component to pass the `todos` signal to `TodoList`:
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
            <TodoList todos={todos.clone()} />
        </section>
    }
}
```

## Updating the `TodoItem` Component

Now, let's modify the `TodoItem` component to accept the `todos` signal and handle the checkbox interaction.

```rust
#[component]
fn TodoItem(todo: Todo, todos: Signal<Vec<Todo>>) -> View {
    let on_toggle = move |_| {
        // Find the todo in the list and toggle its `completed` status
        let updated_todos = todos.0().iter().map(|t| {
            if t.id == todo.id {
                let mut updated_todo = t.clone();
                updated_todo.completed = !t.completed;
                updated_todo
            } else {
                t.clone()
            }
        }).collect();

        todos.1.set(updated_todos);
    };

    rsx! {
        <li class={if todo.completed { "completed" } else { "" }}>
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    checked={todo.completed}
                    on:change={on_toggle}
                />
                <label>{&todo.text}</label>
                <button class="destroy"></button>
            </div>
        </li>
    }
}
```

Let's break down the changes:
1.  The component now accepts a `todos: Signal<Vec<Todo>>` prop.
2.  We've added a `checked` property to the `<input>` element, binding it to the `todo.completed` field. Now the checkbox will correctly reflect the todo's state.
3.  We've added an `on:change` event handler to the checkbox, which calls our `on_toggle` closure.
4.  The `on_toggle` closure creates a new list of todos by iterating over the old one. When it finds the todo that matches the current component's `id`, it creates a new `Todo` with the `completed` status flipped.
5.  Finally, it calls the `todos` signal's setter to update the application state with the newly modified list.
6.  We also added a dynamic `class` to the `<li>` element. If `todo.completed` is `true`, it will add the `completed` class, which you can use to style completed items (e.g., with a line-through).

With these changes, you can now click the checkbox next to a todo, and it will be marked as complete, with its style updating to reflect the change.

In the final part of this tutorial, we'll implement the logic for deleting todos.