use crate::prelude::*;
use dioxus::prelude::*;
use crate::models::{Menus, Submenus};
use crate::components::fastimage::FastImage;
//use dioxus::prelude::*;
use crate::components::nav_item::NavItem;
use crate::models::get_menu_db;
use crate::models::get_submenu_db;
use crate::models::get_single_image_b64;
use crate::Route;
use dioxus::{fullstack::reqwest::Url, prelude::*};
use serde::{Serialize, Deserialize};

use dioxus::prelude::asset;
use crate::document::eval;
use crate::models::get_sliders_db;

#[component]
pub fn ElencoSliders(dir: String) -> Element {
    let d_resource = dir.clone();
    let d = d_resource.clone();
    let dir = use_signal(|| dir.to_string());
    let sliders_res = use_resource(move || get_sliders_db(dir.cloned()));

    let inizializza_slider = move |_| {
        spawn(async move {
            let _ = eval(r#"
                var $slider = $('#example1');
                if ($slider.length > 0 && typeof $.fn.sliderPro !== 'undefined') {
                    $slider.sliderPro({
                        width: 960,
                        height: 500,
                        arrows: true,
                        buttons: true,
                        autoplay: true,
                        autoHeight: false,
                        forceSize: 'none', // Fondamentale per non andare a tutto schermo
                        imageScaleMode: 'cover',
                        centerImage: true
                    });
                }
            "#);
        });
    };

    rsx! {
        match &*sliders_res.read_unchecked() {
            Some(Ok(list)) => rsx! {
                // 1. Contenitore di posizionamento (il "recinto")
                div { 
                    style: "width: 100%; max-width: 960px; margin: 50px auto; position: relative; clear: both;",
                    
                    div { 
                        id: "example1", 
                        class: "slider-pro",
                        onmounted: inizializza_slider, 
                        
                        div { class: "sp-slides",
                            for s in list {
                                div { class: "sp-slide", key: "{s.id}", "{dir}",
                                    FastImage { name: s.img.clone(), dir:{dir.clone()} }
                                    
                                    h3 { class:"sp-layer sp-black sp-padding", "data-horizontal": "40","data-vertical": "10%","data-show-transition": "left","data-hide-transition": "left" ,"{s.titolo}"}
                                    
                                    p { class: "sp-layer sp-white sp-padding hide-medium-screen", "data-horizontal": "40","data-vertical": "22%","data-show-transition": "left","data-hide-transition": "left" , "{s.caption}" }
                                    
                                    p { 
                                        style: "background-color:#330101;color:#ffffff;", 
                                        class: "sp-layer sp-white sp-padding hide-small-screen", 
                                        "data-horizontal": "40","data-vertical": "34%","data-show-transition": "left","data-hide-transition": "left" , 
                                        "{s.testo}" 
                                    }
                                } // Chiusura sp-slide
                            } // Chiusura ciclo for
                        } // Chiusura sp-slides
                    } // Chiusura example1
                } // Chiusura contenitore 960px
            }, 
            _ => rsx! { img { src: CLESSIDRA, id: "header" } }
        } // Chiusura match
    } // Chiusura rsx!
}