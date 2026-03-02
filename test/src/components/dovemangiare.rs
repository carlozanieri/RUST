use crate::prelude::*;
use crate::components::fastimage::FastImage;
use dioxus::prelude::asset;
use crate::models::get_food_db;
#[component]
pub fn Dovemangiare() -> Element {
    let food_res: Resource<std::result::Result<Vec<Foods>, ServerFnError>> = use_resource(move || get_food_db());
rsx! {
    match &*food_res.read_unchecked() {
    Some(Ok(lista)) => rsx! {
    div { 
        class: "popin", 
        style: "background-image: url('/assets/beb/bg.png'); background-repeat: repeat;",
        
        input { 
            id: "popin_check_1", 
            "type": "checkbox", 
            class: "popin_check", 
            hidden: true, 
            checked: "true" 
        }
        
        label { class: "layer fadeIn", "for": "popin_check_1" }
        
        div { 
            class: "content fadeIn", 
            style: "background-image: url('/assets/beb/bg.png'); background-repeat: repeat;",
            
            div { class: "header", 
                "Dove mangiare"
                label { class: "close_popin", "for": "popin_check_1", "X" }
            }
            
            div { class: "main",
            for l in lista {
                div { 
                    style: "display: flex; align-items: center; margin-top: 5%; margin-left: 2%;",
                    FastImage { 
                            name: l.img.clone(), 
                            dir: "ristoranti" 
                        }
                    span { style: "font-size: 1.5em; color: #000000;",
                        b{"{l.titolo}"},",   "," ➡️ {l.indirizzo}","   ","  📞 {l.telefono}","   ", br{}" 🚶‍♀️ Raggiungibile a piedi: { l.apiedi}"
                        
                    }

                }
                br {}
            }   
            }
            
            // In Dioxus, per le rotte interne, usiamo il componente Link 
            // invece del tag <a> per non ricaricare la pagina
            Link { 
                class: "modal-button", 
                to: Route::Home {}, 
                "HOME" 
            }
        }
    }
},
_ => rsx! { img { src: CLESSIDRA, id: "header" } }
}
}
}
