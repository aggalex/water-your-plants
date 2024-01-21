use deadpool_postgres::Pool;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::{delete, get, post, routes, serde::json::Json, Route, State};
use std::ops::Deref;

use crate::business::cdi::transaction::TransactionContext;
use crate::business::cdi::Injects;
use crate::business::http_service::middleware::login::LoggedInUserDTO;
use crate::business::manager::user_manager::UserManager;
use crate::business::manager::ErrorResponse;
use crate::persistence::entity::login::LoginDao;
use crate::persistence::entity::user::{UserInfoDto, UserLoginDto, UsersDao};
use crate::persistence::Transaction;
use uuid::Uuid;

fn logout_cookie(cookie_jar: &CookieJar<'_>) {
    cookie_jar.get("auth").map(|c| cookie_jar.remove(c.clone()));
}

#[post("/login", data = "<login_dto>")]
async fn login(
    login_dto: Json<UserLoginDto>,
    db: &State<Pool>,
    cookie_jar: &CookieJar<'_>,
) -> Result<(), ErrorResponse> {
    let mut manager = db.get().await?;
    let mut context = TransactionContext::new(Transaction::new(&mut manager).await?);

    let auth_key = cookie_jar
        .get_private("auth")
        .map(|cookie| cookie.value().to_string());

    let user_manager: UserManager = context.inject();
    let data = user_manager.login(login_dto.0, auth_key).await?;

    cookie_jar.add_private(Cookie::new("auth", data.key.clone()));

    context.commit(()).await
}

#[post("/logout")]
async fn logout(
    user: LoggedInUserDTO,
    db: &State<Pool>,
    cookie_jar: &CookieJar<'_>,
) -> Result<(), ErrorResponse> {
    let mut manager = db.get().await?;
    let mut context = TransactionContext::new(Transaction::new(&mut manager).await?);

    let user_manager: UserManager = context.inject();
    user_manager.logout(user).await?;

    logout_cookie(cookie_jar);

    context.commit(()).await
}

#[post("/", data = "<login_dto>")]
async fn register(
    login_dto: Json<UserLoginDto>,
    db: &State<Pool>,
) -> Result<Json<i32>, ErrorResponse> {
    let mut manager = db.get().await?;
    let mut context = TransactionContext::new(Transaction::new(&mut manager).await?);

    let user_manager: UserManager = context.inject();
    let info = user_manager.register(login_dto.0).await?;

    context.commit(Json(info.id)).await
}

#[get("/")]
async fn display_info(
    user: LoggedInUserDTO,
    db: &State<Pool>,
) -> Result<Json<UserInfoDto>, ErrorResponse> {
    let mut manager = db.get().await?;
    let mut context = TransactionContext::new(Transaction::new(&mut manager).await?);

    let user_manager: UserManager = context.inject();
    let info = user_manager.get_user_info(user).await?;

    context.commit(Json(info)).await
}

#[delete("/")]
async fn delete_user(
    user: LoggedInUserDTO,
    db: &State<Pool>,
    cookie_jar: &CookieJar<'_>,
) -> Result<(), ErrorResponse> {
    let mut manager = db.get().await?;
    let mut context = TransactionContext::new(Transaction::new(&mut manager).await?);

    let user_manager: UserManager = context.inject();
    user_manager.delete(user).await?;

    context.commit(()).await
}

pub fn routes() -> impl Into<Vec<Route>> {
    routes![login, logout, register, display_info, delete_user]
}
