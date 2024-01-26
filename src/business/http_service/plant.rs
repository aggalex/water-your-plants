use crate::business::cdi::transaction::TransactionContext;
use crate::business::cdi::Injects;
use crate::business::dto::PaginationDto;
use crate::business::http_service::middleware::login::LoggedInUserDTO;
use crate::business::manager::plant_manager::PlantManger;
use crate::business::manager::ErrorResponse;
use crate::persistence::dao::plant::{NewPlantDto, PlantDto};

use crate::persistence::Transaction;
use deadpool_postgres::Pool;

use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes, Route, State};



#[post("/", data = "<plant_dto>")]
async fn new_plant(
    plant_dto: Json<NewPlantDto>,
    db: &State<Pool>,
    logged_in_user_dto: LoggedInUserDTO,
) -> Result<Json<PlantDto>, ErrorResponse> {
    let mut manager = db.get().await?;
    let context = TransactionContext::new(Transaction::new(&mut manager).await?);

    let plant_manager: PlantManger = context.inject();
    let info = plant_manager
        .create_plant(plant_dto.0, logged_in_user_dto)
        .await?;

    context.commit(Json(info)).await
}

#[get("/?<query>&<page>&<page_size>")]
async fn get_plants(
    db: &State<Pool>,
    logged_in_user_dto: LoggedInUserDTO,
    query: Option<String>,
    page: i64,
    page_size: Option<i64>,
) -> Result<Json<Vec<PlantDto>>, ErrorResponse> {
    let mut manager = db.get().await?;
    let context = TransactionContext::new(Transaction::new(&mut manager).await?);

    let pagination = PaginationDto::new_limited(page, page_size)?;

    let plant_manager: PlantManger = context.inject();
    let info = plant_manager
        .get_plants_of_user(query, pagination, logged_in_user_dto)
        .await?;

    context.commit(Json(info)).await
}

#[delete("/<id>")]
async fn delete_plant(
    db: &State<Pool>,
    id: i32,
    logged_in_user_dto: LoggedInUserDTO,
) -> Result<(), ErrorResponse> {
    let mut manager = db.get().await?;
    let context = TransactionContext::new(Transaction::new(&mut manager).await?);

    let plant_manager: PlantManger = context.inject();
    plant_manager.delete_plant(id, logged_in_user_dto).await?;

    context.commit(()).await
}

#[put("/<id>", data = "<new_plant_dto>")]
async fn update_plant(
    db: &State<Pool>,
    new_plant_dto: Json<NewPlantDto>,
    id: i32,
    logged_in_user_dto: LoggedInUserDTO,
) -> Result<(), ErrorResponse> {
    let mut manager = db.get().await?;
    let context = TransactionContext::new(Transaction::new(&mut manager).await?);

    let plant_manager: PlantManger = context.inject();
    plant_manager
        .update_plant(id, new_plant_dto.0, logged_in_user_dto)
        .await?;

    context.commit(()).await
}

pub fn routes() -> impl Into<Vec<Route>> {
    routes![new_plant, get_plants, delete_plant, update_plant]
}
