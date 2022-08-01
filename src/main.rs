#[macro_use]
extern crate diesel;
extern crate dotenv;

use crate::handlers::mats::*;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod handlers;
mod models;
mod schema;
mod config;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = config::load_config();
    let mngr = ConnectionManager::<SqliteConnection>::new(config.database.url);
    let pool: Pool = r2d2::Pool::builder()
        .build(mngr)
        .expect("Failed to create pool.");
    
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(get_items))
            .route("/", web::post().to(create_item))
            .route("/{id}", web::get().to(get_item))
            .route("/{id}", web::delete().to(delete_item))
    })
    .bind("localhost:8000")?
    .run()
    .await
}