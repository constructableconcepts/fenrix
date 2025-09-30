# 4. Deleting Todos

In this final part of the tutorial, we will add the ability for users to delete todos from the list. This will involve adding a click handler to the "destroy" button in our `TodoItem` component.

## Handling the Delete Action

The logic for deleting a todo is very similar to the logic for completing one. We will add a click handler to the destroy button that filters the `todos` list, removing the item that matches the current component's ID.

Let's update the `TodoItem` component one last time:

```rust
#[component]
fn TodoItem(todo: Todo, todos: Signal<Vec<Todo>>) -> View {
    let on_toggle = {
        let todos = todos.clone();
        let todo = todo.clone();
        move |_| {
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
        }
    };

    let on_destroy = move |_| {
        // Filter the list, keeping all todos except this one
        let updated_todos = todos.0()
            .iter()
            .filter(|t| t.id != todo.id)
            .cloned()
            .collect();

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
                <button class="destroy" on:click={on_destroy}></button>
            </div>
        </li>
    }
}
```

We have added an `on_destroy` closure. This closure:
1.  Takes the current list of todos from the signal.
2.  Uses the `filter` method to create a new iterator that includes every todo *except* the one where the `id` matches the current component's `todo.id`.
3.  Collects the results into a new `Vec<Todo>`.
4.  Calls the `todos` signal's setter with this new, smaller list.

We then attach this `on_destroy` closure to the `on:click` event of our `<button class="destroy">`.

## Final Code

Congratulations! You have now built a complete Todo List application with Fenrix. Here is the final code for `src/lib.rs` for your reference:

```rust
use fenrix::prelude::*;
use web_sys::KeyboardEvent;

#[derive(Clone, PartialEq)]
struct Todo {
    id: u32,
    text: String,
    completed: bool,
}

#[component]
fn TodoItem(todo: Todo, todos: Signal<Vec<Todo>>) -> View {
    let on_toggle = {
        let todos = todos.clone();
        let todo = todo.clone();
        move |_| {
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
        }
    };

    let on_destroy = move |_| {
        let updated_todos = todos.0()
            .iter()
            .filter(|t| t.id != todo.id)
            .cloned()
            .collect();
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
                <button class="destroy" on:click={on_destroy}></button>
            </div>
        </li>
    }
}

#[component]
fn TodoList(todos: Signal<Vec<Todo>>) -> View {
    rsx! {
        <section class="main">
            <ul class="todo-list">
                {
                    todos.0().into_iter().map(|todo| {
                        rsx! { <TodoItem todo={todo.clone()} todos={todos.clone()} /> }
                    }).collect::<Vec<_>>()
                }
            </ul>
        </section>
    }
}

#[component]
fn AddTodo(todos: Signal<Vec<Todo>>, next_id: Signal<u32>) -> View {
    let new_todo_text = use_state(String::new);

    let on_keydown = move |event: KeyboardEvent| {
        if event.key() == "Enter" && !new_todo_text.0().is_empty() {
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

#[main]
pub fn main() {
    mount_to_body(App);
}
```

## Next Steps

This tutorial has covered the fundamentals of building an application with Fenrix. From here, you can explore the "Core Concepts" and "API Reference" sections to deepen your understanding of the framework's features. Happy coding!