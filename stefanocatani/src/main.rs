mod config;
mod models;
mod prelude;
mod components;

use crate::prelude::*;
 // Questo caricherà components/mod.rs
use components::casabaldini::Casabaldini;
use components::navbar::Navbar;
use crate::components::dovemangiare::Dovemangiare;

//use components::blog::Blog;
use components::home::Home;
use crate::components::prenotazioni::Prenotazioni;
use crate::components::linkutili::Linkutili;
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/casabaldini/:dir")]
    Casabaldini{dir: String},
     #[route("/dovemangiare")]
    Dovemangiare { },
    #[route("/prenotazioni")]
    Prenotazioni { },
    #[route("/linkutili")]
    Linkutili,
    
}

fn main() {
        
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    
    rsx! {
        document::Script { src: "https://code.jquery.com/jquery-3.6.2.min.js" }
        document::Link { rel: "stylesheet", href: crate::config::EXAMPLE_CSS }
        document::Link { rel: "stylesheet", href: crate::config::MENU_CSS }
        document::Link { rel: "stylesheet", href: crate::config::SLIDERMIN_CSS }
        document::Link { rel: "stylesheet", href: crate::config::SLIDER_CSS }
        document::Script { src: crate::config::JQUERY_JS }
        document::Link { rel: "icon", href: crate::config::FAVICON }
        document:: Meta {name:"viewport", content:"width:device-width, user-scalable:no,initial-scale:1.0, minimum-scale:1.0, maximum-scale:1.0"}
        document::Link { rel: "stylesheet", href: crate::config::MAIN_CSS } 
        document::Link { rel: "stylesheet", href: crate::config::TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: crate::config::POPIN_CSS }
        Router::<Route> {}
    }
}

