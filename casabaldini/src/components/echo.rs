use crate::prelude::*;
//use dioxus::prelude::*;
use crate::models::{Menus, Submenus};
use crate::components::nav_item::NavItem;
use crate::models::get_menu_db;
use crate::models::get_submenu_db;
use crate::models::get_single_image_b64;


#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        div {
            id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput:  move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}
// casabaldini


/// Echo the user input on the server.
#[post("/api/echo")]
pub async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}