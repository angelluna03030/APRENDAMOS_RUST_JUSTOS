use actix_web::{ HttpRequest, Responder };


pub(crate) async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("hola-mundo").unwrap_or("holla mundo que tal a todos ");
    format!("aqui esta mi respuesta:  {}", name)
}