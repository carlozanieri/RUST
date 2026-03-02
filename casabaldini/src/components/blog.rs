use dioxus::prelude::*;
use crate::models::{Menus, Submenus};
use dioxus::prelude::*;
use crate::components::nav_item::NavItem;
use crate::models::get_menu_db;
use crate::models::get_submenu_db;
use crate::models::get_single_image_b64;
use crate::Route;
use dioxus::{fullstack::reqwest::Url, prelude::*};
use serde::{Serialize, Deserialize};

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}