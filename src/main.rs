extern crate diesel;

use diesel::{PgConnection, r2d2::{ConnectionManager, Pool}};
use dotenv::dotenv;
use std::env;
use actix_web::{ App, HttpServer, web::{ self } };
use crate::router::{get_carros, info, insertar_data_base, new_carro};
mod models;
mod schema;
mod router;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
   // print!("database_url: {}", database_url);
    let connection = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(connection)
        .expect("Failed to create pool.");
    let port: u16 = env
        ::var("PORT")
        .expect("PORT must be set")
        .parse()
        .expect("PORT must be a valid u16");
    let host = env::var("HOST").expect("HOST must be set");
    println!("server on port: http://localhost:{}", port);
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
            .service(insertar_data_base)
            .service(info)
            .service(get_carros)
            .service(new_carro)
            .route("/router/{name}", web::get().to(router::greet))
            .route(
                "/",
                web::get().to(|| async { "ok" })
            )
            .route("/{name}", web::get().to(router::greet))
    })
        .bind((host, port))?
        .run().await?;
    Ok(())
}
