use axum::{routing::get, extract::State, Json, Router};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use serde::{Serialize, Deserialize};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir; // <--- Aggiunto questo

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Slider {
    pub id: i64,
    pub img: String, // Questo è il campo 'immagineUrl' nel tuo modello Dart
    pub titolo: String,
    pub testo: String,
    pub caption: String,
}

#[derive(serde::Serialize)]
pub struct FullMenu {
    pub parent: Menus,
    pub children: Vec<Submenus>,
}

#[tokio::main]
async fn main() {
    let db_url = "postgres://carlo:treX39@57.131.31.228:5432/casabaldini";

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("Errore di connessione al database");

    // Definiamo la rotta per i file statici
    // ServeDir::new("static") dice ad Axum di guardare nella cartella ./static
    let static_files_service = ServeDir::new("static");

    let app = Router::new()
        .route("/api/v1/slider", get(get_sliders))
        // Questa riga dice: "Tutto ciò che arriva a /static, cercalo nella cartella static"
        .nest_service("/static", static_files_service) 
        .layer(CorsLayer::permissive())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await.unwrap();
    println!("🚀 API Mobile partita su http://json.casabaldini.eu");
    println!("📂 Immagini servite su http://json.casabaldini.eu/static/img/index/");
    
    axum::serve(listener, app).await.unwrap();
}

async fn get_sliders(State(pool): State<PgPool>) -> Result<Json<Vec<Slider>>, (axum::http::StatusCode, String)> {
    // Nota: Ho aggiunto 'img' nella query perché il tuo modello Rust lo richiede
    let res = sqlx::query_as::<_, Slider>("SELECT id, img, titolo, testo, caption FROM sliders")
        .fetch_all(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(res))
}

pub async fn get_api_menu(pool: web::Data<PgPool>) -> impl Responder {
    let parents = sqlx::query_as::<_, Menus>("SELECT id, codice, radice, livello, titolo, link, ordine FROM menu WHERE livello=2 AND attivo=1 ORDER BY ordine")
        .fetch_all(pool.get_ref()).await.unwrap_or_default();

    let all_sub = sqlx::query_as::<_, Submenus>("SELECT id, codice, radice, livello, titolo, link, ordine FROM submenu WHERE attivo=1 ORDER BY ordine")
        .fetch_all(pool.get_ref()).await.unwrap_or_default();

    let mut response = Vec::new();
    for p in parents {
        let figli: Vec<Submenus> = all_sub.iter()
            .filter(|s| s.radice.trim() == p.codice.trim())
            .cloned()
            .collect();
        response.push(FullMenu { parent: p, children: figli });
    }
    HttpResponse::Ok().json(response)
}