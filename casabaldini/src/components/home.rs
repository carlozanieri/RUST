use crate::components::casabaldini::Casabaldini;
/// Home page
pub use crate::prelude::*;
pub use crate::components::hero::Hero;
pub const BACK_IMG: Asset = asset!("/assets/bgblack.png");
use crate::components::echo::Echo;
use crate::components::linkutili::Linkutili;
#[component]
pub fn Home() -> Element {
    
    rsx! {
        body {
            style: "background-image: url({BACK_IMG}); background-repeat: repeat;",
            Casabaldini { dir: "index" }
                //Echo {}
        // Linkutili {}
        }
    }
}