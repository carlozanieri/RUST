use serde::{Serialize, Deserialize};
use dioxus::{fullstack::reqwest::Url, html::textarea::rows, prelude::*};
#[cfg(feature = "server")]
use sqlx::{PgPool, FromRow};
#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)] 
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Menus {
	pub id:       i64,
	pub codice:   String,
	pub radice:   String,
	pub livello:  i64,
	pub titolo:   String,
	pub link:     String,
    pub ordine:   i64,
	
}
const DB_URL: &str = "postgres://carlo:treX39@57.131.31.228:5432/casabaldini";
#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)] 
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Submenus{
	pub id:       i64,
	pub codice:   String,
	pub radice:   String,
	pub livello:  i64,
	pub titolo:   String,
	pub link:     String,
    pub ordine:   i64,
	
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Slider {
    pub id: i64,
    pub img: String,
    pub titolo: String,
    pub testo: String,
    pub caption: String,
}
#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)] 
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
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

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)] 
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
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

#[server]
pub async fn get_menu_db() -> Result<Vec<Menus>, ServerFnError> {
    // Trasformiamo l'errore di connessione e di query in stringhe leggibili da ServerFnError
    let pool = PgPool::connect(crate::config::DB_URL)
        .await
        .map_err(|e| ServerFnError::new(format!("Errore connessione DB: {}", e)))?;

    let mrows = sqlx::query_as::<_, Menus>("SELECT id, codice,  radice,livello,titolo,link, ordine FROM menu where livello=2 and attivo= 1 order by ordine")
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Errore query: {}", e)))?;
    println!("📡 Server: Row recuperate, invio in corso...");
    Ok(mrows)
}

#[server]
pub async fn get_submenu_db() -> Result<Vec<Submenus>, ServerFnError> {
    // Trasformiamo l'errore di connessione e di query in stringhe leggibili da ServerFnError
    let pools = PgPool::connect(crate::config::DB_URL)
        .await
        .map_err(|e| ServerFnError::new(format!("Errore connessione DB: {}", e)))?;

    let srows = sqlx::query_as::<_, Submenus>("SELECT id, codice,  radice, livello, titolo,link, ordine FROM submenu where attivo = 1 order by ordine")
        .fetch_all(&pools)
        .await
        .map_err(|e| ServerFnError::new(format!("Errore query: {}", e)))?;
    println!("📡 Server: Row recuperate, invio in corso...");
    Ok(srows)
}

#[server]
pub async fn get_sliders_db(dir: String) -> Result<Vec<Slider>, ServerFnError> {
    // Trasformiamo l'errore di connessione e di query in stringhe leggibili da ServerFnError
    let pool = PgPool::connect(crate::config::DB_URL)
        .await
        .map_err(|e| ServerFnError::new(format!("Errore connessione DB: {}", e)))?;
if dir == "index" {
    let pool = PgPool::connect(crate::config::DB_URL)
        .await
        .map_err(|e| ServerFnError::new(format!("Errore connessione DB: {}", e)))?;
    let srows = sqlx::query_as::<_, Slider>("SELECT id, titolo, img, testo, caption FROM sliders WHERE codice = $1") .bind(dir)
                .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Errore query: {}", e)))?;
    println!("📡 Server: Row recuperate, invio in corso...");
    Ok(srows)
}else{
    let pool = PgPool::connect(crate::config::DB_URL)
        .await
        .map_err(|e| ServerFnError::new(format!("Errore connessione DB: {}", e)))?;
    let srows = sqlx::query_as::<_, Slider>("SELECT id, titolo, img, testo, caption FROM sliders WHERE codice2 = $1") .bind(dir)
                .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Errore query: {}", e)))?;
    println!("📡 Server: Row recuperate, invio in corso...");
    Ok(srows)
    }
}


#[server]
pub async fn get_single_image_b64(name: String, dir: String) -> Result<String, ServerFnError> {
    use base64::{Engine as _, engine::general_purpose};
    let path = format!("assets/img/{}/{}",dir, name);
    //let path = format!("assets/img/index/{}", name);
    let bytes = std::fs::read(path).map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(format!("data:image/jpeg;base64,{}", general_purpose::STANDARD.encode(bytes)))
}

#[server]
pub async fn get_link_db() -> Result<Vec<Links>, ServerFnError> {
    // Trasformiamo l'errore di connessione e di query in stringhe leggibili da ServerFnError
    let pool = PgPool::connect(crate::config::DB_URL)
        .await
        .map_err(|e| ServerFnError::new(format!("Errore connessione DB: {}", e)))?;

    let lrows = sqlx::query_as::<_, Links>("SELECT id, codice, img,titolo,descrizione,link, height, width FROM links ")
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Errore query: {}", e)))?;
    println!("📡 Server: Row recuperate, invio in corso...");
    Ok(lrows)
}

#[server]
pub async fn get_food_db() -> Result<Vec<Foods>, ServerFnError> {
    // Trasformiamo l'errore di connessione e di query in stringhe leggibili da ServerFnError
    let pool = PgPool::connect(crate::config::DB_URL)
        .await
        .map_err(|e| ServerFnError::new(format!("Errore connessione DB: {}", e)))?;

    let frows = sqlx::query_as::<_, Foods>("SELECT id, codice, img,titolo,descrizione,link,  width, height, indirizzo, telefono, apiedi FROM food ")
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Errore query: {}", e)))?;
    println!("📡 Server: Row recuperate, invio in corso...");
    Ok(frows)
}