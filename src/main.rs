mod router;
use dotenv::dotenv;
use std::env;
use actix_web::{ App, HttpServer, web::{ self } };

use crate::router::{info, insertar_data_base};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port: u16 = env
        ::var("PORT")
        .expect("PORT must be set")
        .parse()
        .expect("PORT must be a valid u16");
    let host = env::var("HOST").expect("HOST must be set");
    println!("server on port: http://localhost:{}", port);
    HttpServer::new(|| {
        App::new()
            .service(insertar_data_base)
            .service(info)
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
