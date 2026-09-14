mod greet;
use dotenv::dotenv;
use std::env;
use actix_web::{ App, HttpServer, web::{ self } };

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
            .route("/hola", web::get().to(greet::greet))
            .route(
                "/",
                web::get().to(|| async { "ok" })
            )
            .route("/{name}", web::get().to(greet::greet))
    })
        .bind((host, port))?
        .run().await?;
    Ok(())
}
