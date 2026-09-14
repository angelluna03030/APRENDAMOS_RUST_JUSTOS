use actix_web::{ HttpRequest, HttpResponse, Responder, get, post, web };
use std::sync::{ Mutex, OnceLock };

static DATA_BASE: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

fn inser_data_base(data: String) {
    let db = DATA_BASE.get_or_init(|| Mutex::new(Vec::new()));
    let mut db = db.lock().unwrap();
    db.push(data);
}

fn get_data_base() -> &'static Mutex<Vec<String>> {
    return DATA_BASE.get_or_init(|| Mutex::new(Vec::new()));
}

pub(crate) async fn greet(req: HttpRequest) -> impl Responder {
    let _name = req.match_info().get("name").unwrap_or("holla mundo que tal a todos ");
    let db = get_data_base().lock().unwrap();
    format!("aqui esta tu ruta: {}", db.join(", "))
}

#[post("/echo")]
pub(crate) async fn insertar_data_base(data: web::Bytes) -> impl Responder {
    let data = String::from_utf8_lossy(&data).to_string();
    inser_data_base(data.clone());
    HttpResponse::Ok().body(format!("me a llegodo {}", data))
}

#[get("/data")]
pub(crate) async fn info() -> impl Responder {
    let db = get_data_base().lock().unwrap();
    HttpResponse::Ok().body(db.join(", "))
}
