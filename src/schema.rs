// @generated automatically by Diesel CLI.
#![allow(non_snake_case)]
diesel::table! {
    carros (id) {
        id -> Int4,
        cliente_id -> Int4,
        #[max_length = 10]
        placa -> Varchar,
        #[max_length = 50]
        marca -> Varchar,
        #[max_length = 50]
        modelo -> Nullable<Varchar>,
    }
}

diesel::table! {
    clientes (id) {
        id -> Int4,
        #[max_length = 100]
        nombre -> Varchar,
        #[max_length = 20]
        telefono -> Nullable<Varchar>,
    }
}

diesel::table! {
    reparaciones (id) {
        id -> Int4,
        carro_id -> Int4,
        descripcion -> Text,
        #[max_length = 30]
        estado -> Nullable<Varchar>,
        precio -> Nullable<Numeric>,
    }
}

diesel::joinable!(carros -> clientes (cliente_id));
diesel::joinable!(reparaciones -> carros (carro_id));

diesel::allow_tables_to_appear_in_same_query!(carros, clientes, reparaciones,);
