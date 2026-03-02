#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use dioxus::document::eval;
use axum::Router;
use tower_http::services::ServeDir;
use axum::routing::get_service;
use base64::Engine;

 
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, sqlx::FromRow)] // <--- Aggiungi sqlx::FromRow qui

pub struct Slider {
    pub id: i32,
    pub img: String,
    pub titolo: String,
    pub caption: String,
    pub testo: String,
    pub img_path: String,
}
pub struct SliderOut {
    pub id: i32,
    pub img: String,
    pub titolo: String,
    pub caption: String,
    pub testo: String,
    pub img_path: String,
}


const SLIDER_CSS: Asset = asset!("/assets/home/dist/css/slider-pro.min.css");
const EXAMPLES_CSS: Asset = asset!("/assets/home/dist/css/examples.css");
const SLIDER_JS: Asset = asset!("/assets/home/dist/js/jquery.sliderPro.min.js");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
//const SLIDER_SVG: Asset = asset!("/assets/img/index/cafaggiolo.jpg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
fn main() {
    let app = Router::new()
        .nest_service("/assets", ServeDir::new("public"))
        .fallback(dioxus_axum::render_app(App))
        // 🔴 QUESTA RIGA RISOLVE L’ERRORE
        .with_state(());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[component]

    
   
pub fn App() -> Element {
    let sliders = use_server_future(get_sliders)?;
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: SLIDER_CSS }
        document::Link { rel: "stylesheet", href: EXAMPLES_CSS }
        document::Script { src: "https://code.jquery.com/jquery-3.6.2.min.js" }
        document::Script { src: SLIDER_JS }

        div { class: "container",
            match sliders.value()() {
                Some(Ok(sliders)) => rsx! {
                    div {
    for s in &sliders {
    div { 
        class: "container-immagine",
        h3 { "{s.titolo}" }
        
        // Chiamiamo il test
        img { 
    // Proviamo a passare il percorso come una stringa pura, 
    // sperando che il server Axum sottostante la veda.
    src: "{s.img}", 
    width: "400px" 
}
    img {
    src: format!("/assets/img/index/{}", s.img),
    id: "header"
}
        p { "{s.testo}" }
    }


}
                    }
                },
                Some(Err(e)) => rsx! { "Errore: {e}" },
                None => rsx! { "Caricamento dati..." }
            }
        }
    }
}

#[server]
pub async fn get_image(path: String) -> Result<Vec<u8>, ServerFnError> {
    // Percorso relativo alla radice del tuo progetto
    
    
    let full_path = format!("/assets/img/index/{}", path.replace("..", ""));
    let bytes = std::fs::read(&full_path).map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(bytes)
    
}
pub async fn get_sliders() -> Result<Vec<Slider>, ServerFnError> {
    use sqlx::SqlitePool;
    
    let pool = SqlitePool::connect("sqlite:casabaldini.sqlite").await
        .map_err(|e| ServerFnError::new(&format!("DB Connection Error: {}", e)))?;
    
    // Rimuoviamo il "!" da query_as per evitare il controllo a tempo di compilazione
    let res = sqlx::query_as::<_, Slider>("SELECT id, img, titolo, caption, testo FROM sliders")
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(&format!("Query Error: {}", e)))?;
let res = res
    .into_iter()
    
    .collect();
    Ok(res)
}

#[server]
pub async fn get_image_url(name: String) -> Result<String, ServerFnError> {
    // Invece di leggere i byte, restituiamo un percorso.
    // Ma attenzione: qui sta il trucco. 
    // Se il server non serve la cartella, il browser non la vedrà mai.
    Ok(format!("/assets/img/index/{}", name))
}
#[component]
fn ImmagineDinamica(nome: String) -> Element {
    let risorsa_immagine = use_resource(move || {
        let n = nome.clone();
        async move { get_image(n).await }
    });

    // PASSO 1: Estraiamo i dati clonandoli e chiudiamo subito il prestito
    // .cloned() trasforma un &Result in un Result clonato (Vec<u8> implementa Clone)
    let risultato_opzionale = risorsa_immagine.read().clone();

    // PASSO 2: Ora la risorsa è "libera". Lavoriamo sulla nostra copia locale.
    match risultato_opzionale {
        Some(Ok(bytes)) => {
            // Trasformiamo i byte in Base64
            let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
            rsx! {
                img { 
                    src: "data:image/jpeg;base64,{b64}",
                    class: "header",
                    width: "400px" 
                }
            }
        },
        Some(Err(e)) => rsx! { p { "Errore caricamento: {e}" } },
        None => rsx! { p { "Caricamento immagine..." } }
    }
}

fn server_specific_config() -> impl FnOnce(&mut Router) + Send + Sync + 'static {
    move |router: &mut Router| {
        // "Montiamo" la cartella fisica 'assets' sulla rotta web '/assets'
        *router = router.clone().nest_service(
            "/assets", 
            ServeDir::new("assets")
        );
    }
}
#[component]
fn ImmagineDalDb(nome_file: String) -> Element {
    let risorsa_immagine = use_resource(move || {
        let n = nome_file.clone();
        async move { get_image(n).await }
    });

    // 1. Estraiamo il valore e CLONIAMO subito. 
    // Questo chiude il "prestito" (borrow) immediatamente.
    let valore_clonato = risorsa_immagine.read().clone();

    // 2. Ora il match lavora su 'valore_clonato' (una copia locale), 
    // non su un riferimento alla risorsa. Il compilatore sarà felice.
    match valore_clonato {
        Some(Ok(bytes)) => {
            let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
            rsx! {
                img { 
                    src: "data:image/jpeg;base64,{b64}",
                    class: "header",
                    width: "400px" 
                }
            }
        },
        Some(Err(e)) => rsx! { p { "Errore: {e}" } },
        None => rsx! { p { "Caricamento immagine..." } }
    }
}
#[component]
fn ImmagineDinamicaTest() -> Element {
    // Una stringa Base64 che rappresenta un pixel rosso. 
    // È a tutti gli effetti una "Stringa" come i tuoi titoli dal DB.
    let pixel_rosso = "R0lGODlhAQABAIAAAAD/AP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7";
    
    rsx! {
        div {
            style: "border: 2px solid black; padding: 10px; margin: 5px;",
            p { "Test Dinamico:" }
            img { 
                src: "data:image/gif;base64,{pixel_rosso}",
                width: "50px",
                height: "50px"
            }
        }
    }
}