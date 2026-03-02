use dioxus::prelude::*;
use crate::models::{Menus, Submenus};
use crate::components::nav_item::NavItem;
use crate::models::get_menu_db;
use crate::models::get_submenu_db;
use crate::Route; // Se usi le rotte per i link, servirà anche questo
#[component]
pub fn Navbar() -> Element {
    let menu_res = use_resource(move || get_menu_db());
    let submenu_res = use_resource(move || get_submenu_db());
    
    // Stato per l'apertura su mobile\
    let mut is_open = use_signal(|| false);

    rsx! {
        div { class: "sp-menu",
            // Il bottone ora ha un evento onclick Rust
            div {
                class: "menu-toggle",
                style: "position: absolute; top: -20px;",
                button {
                    r#type: "button",
                    id: "menu-btn",
                    onclick: move |_| is_open.toggle(), // Inverte lo stato vero/falso
                    span { class: "icon-bar" }
                    span { class: "icon-bar" }
                    span { class: "icon-bar" }
                }
            }

            // Applichiamo la classe "show" se is_open è vero
            ul {
                id: "respMenu",
                class: if is_open() { "dioxus-menu show" } else { "dioxus-menu" },

                match (&*menu_res.read_unchecked(), &*submenu_res.read_unchecked()) {
                    (Some(Ok(parents)), Some(Ok(all_subitems))) => {
                        rsx! {
                            for m in parents {
                                {
                                    let figli = all_subitems
                                        .iter()
                                        .filter(|s| s.radice.trim() == m.codice.trim())
                                        .cloned()
                                        .collect::<Vec<Submenus>>();
                                    rsx! {
                                        NavItem { key: "{m.id}", m: m.clone(), subitems: figli }
                                    }
                                }
                            }
                        }
                    }
                    _ => rsx! {
                        li { "Caricamento..." }
                    },
                }
            }
        }
        Outlet::<Route> {}
    }
}