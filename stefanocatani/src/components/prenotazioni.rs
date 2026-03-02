use crate::prelude::*;

#[component]
pub fn Prenotazioni() -> Element {
rsx! {
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
                "Prenotazioni"
                label { class: "close_popin", "for": "popin_check_1", "X" }
            }
            
            div { class: "main",
                div { 
                    style: "display: flex; align-items: center; margin-top: 5%; margin-left: 2%;",
                    img { 
                        style: "width: 10%; margin-right: 10px;", 
                        src: TEL_JPG 
                    }
                    span { style: "font-size: 1.5em; color: #000000;",
                        "Telefonando al "
                        b { "+39 3207060411" }
                    }
                }
                br {}
                div { 
                    style: "display: flex; align-items: center; margin-top: 5%; margin-left: 2%;",
                    img { 
                        style: "width: 10%; margin-right: 10px;", 
                        src: MAIL_JPG 
                    }
                    span { style: "font-size: 1.5em; color: #000000;",
                        "Inviando un E-Mail a "
                        b { "bruna.bbaldini@gmail.com" }
                    }
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
}
}
