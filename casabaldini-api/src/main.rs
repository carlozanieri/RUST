use axum::{routing::get, extract::{State, Query}, Json, Router};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use serde::{Serialize, Deserialize};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir; // <--- Aggiunto questo
use axum::response::IntoResponse;
//use axum::extract::{State, Query};
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Slider {
    pub id: i64,
    pub img: String, // Questo è il campo 'immagineUrl' nel tuo modello Dart
    pub titolo: String,
    pub testo: String,
    pub caption: String,
}
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Menus {
    pub id: i64,
    pub codice: String,
    pub radice: String,
    pub livello: i64,
    pub titolo: String,
    pub link: String,
    pub ordine: i64,
    pub tipopage: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Submenus {
    pub id: i64,
    pub codice: String,
    pub radice: String,
    pub livello: i64,
    pub titolo: String,
    pub link: String,
    pub ordine: i64,
    pub tipopage: String,
}
#[derive(serde::Serialize)]
pub struct FullMenu {
    pub parent: Menus,
    pub children: Vec<Submenus>,
}
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Foods {
	pub id:       i64,
	pub codice:   String,
	pub img:      String,
	pub titolo:   String,
    pub descrizione:     String,
	pub link:     String,
    pub width:   String,
    pub height:   String,
    pub indirizzo:   String,
	pub telefono:    String,
	pub apiedi:   String,
	
}
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Links {
	pub id:       i64,
	pub codice:   String,
	pub img:      String,
	pub titolo:   String,
    pub descrizione:     String,
	pub link:     String,
    pub height:   String,
    pub width:   String,
	
}

#[derive(serde::Deserialize)]
pub struct SliderParams {
    pub dir: Option<String>, // Usiamo Option per sicurezza se manca il parametro
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
        .route("/api/v1/slider", get(get_api_sliders))
        .route("/api/v1/menu", get(get_api_menu))
        .route("/api/v1/foods", get(get_api_food))
        .route("/api/v1/links", get(get_api_links))
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

pub async fn get_api_menu(State(pool): State<PgPool>) -> impl IntoResponse {
    let parents = sqlx::query_as::<_, Menus>(
        "SELECT id, codice, radice, livello, titolo, link, ordine,tipopage FROM menu WHERE livello=2 AND attivo=1 ORDER BY ordine"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let all_sub = sqlx::query_as::<_, Submenus>(
        "SELECT id, codice, radice, livello, titolo, link, ordine, tipopage FROM submenu WHERE attivo=1 ORDER BY ordine"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let mut response = Vec::new();
    for p in parents {
        let figli: Vec<Submenus> = all_sub
            .iter()
            .filter(|s| s.radice.trim() == p.codice.trim())
            .cloned()
            .collect();
            
        response.push(FullMenu {
            parent: p,
            children: figli,
        });
    }

    // In Axum restituiamo semplicemente una tupla o un oggetto Json
    Json(response)
}

pub async fn get_api_sliders(
    State(pool): State<PgPool>,
    Query(params): Query<SliderParams>,
) -> impl IntoResponse {
    // Se dir manca, usiamo "index" come default
    let dir = params.dir.unwrap_or_else(|| "index".to_string());
    
    // Logica: se dir è "index" usa 'codice', altrimenti 'codice2'
    let query = if dir == "index" {
        "SELECT id, titolo, img, testo, caption FROM sliders WHERE codice = $1"
    } else {
        "SELECT id, titolo, img, testo, caption FROM sliders WHERE codice2 = $1"
    };

    let srows = sqlx::query_as::<_, Slider>(query)
        .bind(&dir)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    Json(srows)
}

pub async fn get_api_food(State(pool): State<PgPool>) -> Result<Json<Vec<Foods>>, (axum::http::StatusCode, String)> {
    // Nota: Ho aggiunto 'img' nella query perché il tuo modello Rust lo richiede
    let res = sqlx::query_as::<_, Foods>("SELECT id, codice, img,titolo,descrizione,link,  width, height, indirizzo, telefono, apiedi FROM food ")
        .fetch_all(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(res))
}

pub async fn get_api_links(State(pool): State<PgPool>) -> Result<Json<Vec<Links>>, (axum::http::StatusCode, String)> {
    // Nota: Ho aggiunto 'img' nella query perché il tuo modello Rust lo richiede
    let res = sqlx::query_as::<_, Links>("SELECT id, codice, img,titolo,descrizione,link, height, width FROM links ")
        .fetch_all(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(res))
}