use std::ops::Deref;
use deadpool_postgres::Pool;
use rocket::{post, State};
use rocket::http::{CookieJar, Status};
use rocket::serde::json::Json;
use uuid::Uuid;
use crate::persistence::entity::plant::{NewPlantDto, PlantDto};
use crate::persistence::entity::user::{UserLoginDto, UsersDao};

#[post("/", data = "<plant_dto>")]
pub async fn new_plant(plant_dto: Json<NewPlantDto>, db: &State<Pool>, _cookie_jar: &CookieJar<'_>) -> Result<Json<i32>, Status> {
    Ok(Json(0))
}
