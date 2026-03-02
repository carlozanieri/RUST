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

    rsx! {
        div { class: "sp-menu",
            div { class: "menu-toggle", style: "position: absolute; top: -20px;",
                button { type: "button", id: "menu-btn",
                    span { class: "icon-bar" }
                    span { class: "icon-bar" }
                    span { class: "icon-bar" }
                }
            }
            
            ul { id: "respMenu", class: "dioxus-menu",
                // Inizio blocco dati
                {
                    match (&*menu_res.read_unchecked(), &*submenu_res.read_unchecked()) {
                        (Some(Ok(parents)), Some(Ok(all_subitems))) => {
                            rsx! {
                                for m in parents {
    {
        // Dentro il ciclo for m in parents...
let figli = all_subitems.iter()
    .filter(|s| s.radice.trim() == m.codice.trim())
    .cloned()
    .collect::<Vec<Submenus>>();

rsx! { 
    NavItem {
        key: "{m.id}", 
        m: m.clone(), 
        subitems: figli // Usa il nuovo nome qui
    } 
}
    }
}
                            }
                        },
                        _ => rsx! { li { "Caricamento..." } }
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}