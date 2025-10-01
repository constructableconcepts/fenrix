use fenrix_core::create_signal;
use fenrix_macros::{component, rsx, server};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::Node;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}

#[server]
pub async fn get_user_from_db(id: u32) -> Result<User, String> {
    // This is the server-side implementation.
    // It will only be included in the server binary.
    println!("Server: Received request for user with id: {}", id);
    if id == 1 {
        Ok(User {
            id: 1,
            name: "GV".to_string(),
        })
    } else {
        Err("User not found".to_string())
    }
}

#[component]
pub fn App() -> Node {
    let (user, set_user) = create_signal(None::<User>);

    let handle_click = {
        let set_user = set_user.clone();
        move |_: web_sys::MouseEvent| {
            let set_user = set_user.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match get_user_from_db(1).await {
                    Ok(fetched_user) => {
                        set_user(Some(fetched_user));
                    }
                    Err(e) => {
                        web_sys::console::error_1(&e.into());
                    }
                }
            });
        }
    };

    rsx! {
        <div>
            <h1>"Server Function Example"</h1>
            <button (click)={handle_click}>"Fetch User from Server"</button>
            <div>
                {
                    if let Some(user) = user() {
                        format!("Fetched User: {} (ID: {})", user.name, user.id)
                    } else {
                        "No user fetched yet.".to_string()
                    }
                }
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    let app = App();
    fenrix_dom::render(app);
}