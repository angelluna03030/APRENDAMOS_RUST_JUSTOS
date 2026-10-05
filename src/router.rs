use actix_web::{ HttpRequest, HttpResponse, Responder, get, post, web };
use std::sync::{ Mutex, OnceLock };
use diesel::pg::PgConnection;
use serde_json::json;
use diesel::r2d2::{ self };
use diesel::r2d2::{ ConnectionManager };
use crate::models::carros::{ NewcarrosHandler, CarroModel };

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
    // la funcion un unwarp() funcion que espra un valor y no exite el valor se cierra el programa
    let db = get_data_base().lock().unwrap();
    HttpResponse::Ok().body(db.join(", "))
}

#[post("/api/carro/newcarro")]
pub(crate) async fn new_carro(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    item: web::Json<NewcarrosHandler>
) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al obtenr la conexio ");
    match web::block(move || CarroModel::add_carros(&mut conn, &item)).await{
        Ok(data) =>{
            let data = data.unwrap();
            HttpResponse::Ok().json(json!(data))
        }
            Err(err) => HttpResponse::Ok().body(err.to_string()),
    }
}


#[get("/api/carro")]
pub(crate) async fn get_carros(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al obtenr la conexio ");
    match web::block(move || CarroModel::get_carros(&mut conn)).await{
        Ok(data) =>{
            let data = data.unwrap();
            HttpResponse::Ok().json(json!(data))
        }
            Err(err) => HttpResponse::Ok().body(err.to_string()),
    }
}


