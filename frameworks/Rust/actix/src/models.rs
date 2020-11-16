use diesel::Queryable;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub message: &'static str,
}

#[allow(non_snake_case)]
#[derive(Serialize, Queryable, FromRow, Debug)]
pub struct World {
    pub id: i32,
    pub randomnumber: i32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Queryable, FromRow, Debug)]
pub struct Fortune {
    pub id: i32,
    pub message: String,
}
