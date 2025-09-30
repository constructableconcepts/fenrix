# 1. Setup & Basic Components

Welcome to the first part of the Fenrix Todo List tutorial. In this section, we'll set up a new Fenrix project and create the basic components that will form the structure of our application.

## Create a New Project

First, let's create a new Fenrix project using the CLI. If you haven't installed the CLI yet, please refer to the [Installation guide](../../getting-started/installation.md).

```bash
fenrix-cli new fenrix-todo
cd fenrix-todo
```

This will create a new project with the default "Hello, World!" template. Open the `src/lib.rs` file. We'll be replacing the existing code with our own.

## The Main App Component

Let's start by defining the main `App` component. This will be the root of our application and will contain all other components.

Clear the contents of `src/lib.rs` and add the following code:

```rust
use fenrix::prelude::*;

#[component]
fn App() -> View {
    rsx! {
        <section class="todoapp">
            <header class="header">
                <h1>"todos"</h1>
                // Input for new todos will go here
            </header>
            // The list of todos will go here
        </section>
    }
}

#[main]
pub fn main() {
    mount_to_body(App);
}
```

This sets up the basic HTML structure for our app, including a header with the title. We've left placeholders for where the new todo input and the list of todos will eventually go.

*(Note: For this tutorial, we assume you have some basic CSS styles for the classes like `todoapp`, `header`, etc. You can find many examples of "TodoMVC" CSS online to use in your `index.html` file.)*

## The `TodoItem` Component

Next, let's create a component to represent a single todo item in our list. For now, it will be a static component that just displays some placeholder text.

Add the following component to `src/lib.rs`:

```rust
#[component]
fn TodoItem() -> View {
    rsx! {
        <li>
            <div class="view">
                <input class="toggle" type="checkbox" />
                <label>{"A static todo item"}</label>
                <button class="destroy"></button>
            </div>
        </li>
    }
}
```

This component renders a list item (`<li>`) with a checkbox, a label for the todo text, and a destroy button.

## The `TodoList` Component

Now, let's create a component that will be responsible for rendering a list of `TodoItem` components.

```rust
#[component]
fn TodoList() -> View {
    rsx! {
        <section class="main">
            <ul class="todo-list">
                <TodoItem />
                <TodoItem />
                <TodoItem />
            </ul>
        </section>
    }
}
```

For now, our `TodoList` component simply renders three instances of our static `TodoItem` component. In the next parts of this tutorial, we will make this dynamic.

## Putting It All Together

Finally, let's add the `TodoList` component to our main `App` component.

Update the `App` component like so:

```rust
#[component]
fn App() -> View {
    rsx! {
        <section class="todoapp">
            <header class="header">
                <h1>"todos"</h1>
                // Input for new todos will go here
            </header>
            <TodoList />
        </section>
    }
}
```

Now, your complete `src/lib.rs` should look like this:

```rust
use fenrix::prelude::*;

#[component]
fn TodoItem() -> View {
    rsx! {
        <li>
            <div class="view">
                <input class="toggle" type="checkbox" />
                <label>{"A static todo item"}</label>
                <button class="destroy"></button>
            </div>
        </li>
    }
}

#[component]
fn TodoList() -> View {
    rsx! {
        <section class="main">
            <ul class="todo-list">
                <TodoItem />
                <TodoItem />
                <TodoItem />
            </ul>
        </section>
    }
}

#[component]
fn App() -> View {
    rsx! {
        <section class="todoapp">
            <header class="header">
                <h1>"todos"</h1>
                // Input for new todos will go here
            </header>
            <TodoList />
        </section>
    }
}

#[main]
pub fn main() {
    mount_to_body(App);
}
```

If you run `fenrix-cli dev`, you should now see the basic structure of your Todo app, with a title and a list of three static todo items.

In the next part, we'll introduce state management to allow users to add new todos to the list.