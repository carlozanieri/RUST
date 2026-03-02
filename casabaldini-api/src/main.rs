use axum::{routing::get, extract::State, Json, Router};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use serde::{Serialize, Deserialize};
use tower_http::cors::CorsLayer;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Slider {
pub id: i64,
pub img: String,
pub titolo: String,
pub testo: String,
pub caption: String,
}

#[tokio::main]
async fn main() {
// Ricordati di mettere le tue credenziali reali qui
let db_url = "postgres://carlo:treX39@57.131.31.228:5432/casabaldini";

let pool = PgPoolOptions::new()
.max_connections(5)
.connect(db_url)
.await
.expect("Errore di connessione al database");

let app = Router::new()
.route("/api/v1/slider", get(get_sliders))
.layer(CorsLayer::permissive())
.with_state(pool);

let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await.unwrap();
println!("🚀 API Mobile partita su http://localhost:3333");
axum::serve(listener, app).await.unwrap();
}

// Questa è la funzione che mancava o che il compilatore non trovava:
async fn get_sliders(State(pool): State<PgPool>) -> Result<Json<Vec<Slider>>, (axum::http::StatusCode, String)> {
let res = sqlx::query_as::<_, Slider>("SELECT id, titolo, img, testo, caption FROM sliders")
.fetch_all(&pool)
.await
.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

Ok(Json(res))
}

