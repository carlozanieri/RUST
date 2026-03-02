use crate::prelude::*;
pub use crate::models::get_sliders_db;
use crate::components::linkutili::Linkutili;
use crate::components::elencosliders::ElencoSliders;
pub const BACK_IMG: Asset = asset!("/assets/bgblack.png");

#[component]
pub fn Casabaldini(dir: String) -> Element {
    println!("--- ESECUZIONE COMPONENTE ---");
    pub const BACK_IMG: Asset = asset!("/assets/bgblack.png");
    let background_image = "{BACK_IMG}";
    let dir = use_signal(|| dir.to_string());
    //let d_resource = dir.clone();
    //let d = d_resource.clone();
    let sliders = use_resource(move || get_sliders_db(dir.cloned()));
    rsx! {
        body { style: "background-image: url({BACK_IMG}); background-repeat: repeat;",
            div { class: "slider-pro",

                hr {}
                ElencoSliders { dir }
                Linkutili {}
            }
        }
    }
}