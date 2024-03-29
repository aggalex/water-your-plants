use crate::business::cdi::transaction::TransactionContext;
use crate::business::cdi::Injects;
use crate::business::dto::PaginationDto;
use crate::business::http_service::middleware::login::LoggedInUserDTO;

use crate::business::manager::plant_profile_manager::PlantProfileManager;
use crate::business::manager::ErrorResponse;

use crate::persistence::dao::plant_profile::{NewPlantProfileDto, PlantProfileDto};

use crate::persistence::Transaction;
use deadpool_postgres::Pool;

use rocket::serde::json::Json;
use rocket::{get, post, routes, Route, State};



#[post("/", data = "<profile_dto>")]
async fn create_plant_profile(
    profile_dto: Json<NewPlantProfileDto>,
    db: &State<Pool>,
    _logged_in_user_dto: LoggedInUserDTO,
) -> Result<Json<PlantProfileDto>, ErrorResponse> {
    let mut manager = db.get().await?;
    let context = TransactionContext::new(Transaction::new(&mut manager).await?);

    let plant_profile_manager: PlantProfileManager = context.inject();
    let info = plant_profile_manager.create_profile(profile_dto.0).await?;

    context.commit(Json(info)).await
}

#[get("/?<query>&<page>&<page_size>")]
async fn get_profiles(
    db: &State<Pool>,
    query: Option<String>,
    page: i64,
    page_size: Option<i64>,
) -> Result<Json<Vec<PlantProfileDto>>, ErrorResponse> {
    let mut manager = db.get().await?;
    let context = TransactionContext::new(Transaction::new(&mut manager).await?);

    let pagination_dto = PaginationDto::new_limited(page, page_size)?;

    let plant_profile_manager: PlantProfileManager = context.inject();
    let info = plant_profile_manager
        .get_profiles(query, pagination_dto)
        .await?;

    context.commit(Json(info)).await
}

pub fn routes() -> impl Into<Vec<Route>> {
    routes![create_plant_profile, get_profiles]
}
