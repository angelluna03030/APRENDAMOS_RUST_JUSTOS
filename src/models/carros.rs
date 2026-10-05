use super::super::schema::carros;
use super::super::schema::carros::dsl::*;
use diesel::prelude::*;
use serde::{ Deserialize, Serialize };

#[derive(Insertable, Debug)]
#[diesel(table_name = carros)]
pub struct Newcarros<'a> {
    pub cliente_id: &'a i32,
    pub placa: &'a String,
    pub marca: &'a String,
    pub modelo: Option<&'a String>,
}

//ahora el handler, lo necesitamos para las apis
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewcarrosHandler {
    pub cliente_id: i32,
    pub placa: String,
    pub marca: String,
    pub modelo: String,
}
//Ahora el modelo de la base de datos
#[derive(Queryable, Selectable, Debug, Deserialize, Serialize)]
#[diesel(table_name = carros)]
pub struct CarroModel {
    pub id: i32,
    pub cliente_id: i32,
    pub placa: String,
    pub marca: String,
    pub modelo: Option<String>,
}

impl CarroModel {
    pub fn get_carros(conn: &mut PgConnection) -> Result<Vec<CarroModel>, diesel::result::Error> {
        let cars = carros.select(CarroModel::as_select()).load::<CarroModel>(conn);
        cars
    }

    pub fn add_carros<'a>(
        conn: &'a mut PgConnection,
        car: &'a NewcarrosHandler
    ) -> Result<CarroModel, diesel::result::Error> {
        let new_cards = Newcarros {
            cliente_id: &car.cliente_id,
            placa: &car.placa,
            marca: &car.marca,
            modelo: Some(&car.modelo),
        };
        diesel
            ::insert_into(carros::table)
            .values(new_cards)
            .returning(CarroModel::as_returning())
            .get_result::<CarroModel>(conn)
    }
}
