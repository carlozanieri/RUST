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
pub fn FastImage(name: String, dir: String) -> Element {
    let mut img_data = use_signal(|| String::new());
    let n_resource = name.clone();
    let d_resource = dir.clone();
    use_resource(move || {
        let n = n_resource.clone();
        let d = d_resource.clone();
        async move {
            if let Ok(b64) = get_single_image_b64(n, d).await {
                img_data.set(b64);
            }
        }
    });

    rsx! { 
        // Se l'immagine c'è, la mostriamo senza classi dello slider
        if !img_data().is_empty() {
            if dir == "links" || dir == "ristoranti" {
            img { 
                key: "{name}",
                src: "{img_data}", 
                // Usiamo stili brutali per essere sicuri che esistano
                style: "width: 90px; height: 56px; display: block !important; visibility: visible !important; opacity: 1 !important;"
            }
            }else{
                img { 
                key: "{name}",
                src: "{img_data}", 
                // Usiamo stili brutali per essere sicuri che esistano
                style: "width: 960px; height: 520px; display: block !important; visibility: visible !important; opacity: 1 !important;"
            }
             }
        } 
    }
}