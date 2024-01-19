use std::ops::Deref;
use deadpool_postgres::Pool;
use rocket::{delete, get, post, serde::json::Json, State};
use rocket::http::{Cookie, CookieJar, Status};


use uuid::Uuid;
use crate::business::DbExtensions;
use crate::business::middleware::login::Login;
use crate::persistence::entity::login::LoginDao;
use crate::persistence::entity::user::{UserInfoDto, UserLoginDto, UsersDao};

#[post("/user/login", data = "<login_dto>")]
pub async fn login(login_dto: Json<UserLoginDto>, db: &State<Pool>, cookie_jar: &CookieJar<'_>) -> Result<(), Status> {
    let mut manager = db.deref().get().await;
    let tx = manager.as_mut().get_transaction().await?;
    let user_dao = UsersDao::from(&tx);
    let login_dao = LoginDao::from(&tx);

    if !cookie_jar.get_private("auth").is_none() {
        return Err(Status::BadRequest)
    };

    let Some(user) = user_dao.find_by_username(&login_dto.username).await.map_err(Into::into)? else {
        return Err(Status::BadRequest)
    };

    let Ok(true) = bcrypt::verify(&login_dto.password, &user.password) else {
        return Err(Status::BadRequest)
    };

    let login = login_dao.login(user.id).await.map_err(Into::into)?;

    cookie_jar.add_private(Cookie::new("auth", login.key));

    tx.commit().await.map_err(Into::into)?;

    Ok(())
}

#[post("/user/logout")]
pub async fn logout(user: Login, db: &State<Pool>, cookie_jar: &CookieJar<'_>) -> Result<(), Status> {
    let mut manager = db.deref().get().await;
    let tx = manager.as_mut().get_transaction().await?;
    let login_dao = LoginDao::from(&tx);

    login_dao.logout(&user.key).await.map_err(Into::into)?;

    cookie_jar.get("auth").map(|c| cookie_jar.remove(c.clone()));

    Ok(())
}

#[post("/user", data = "<login_dto>")]
pub async fn register(login_dto: Json<UserLoginDto>, db: &State<Pool>, _cookie_jar: &CookieJar<'_>) -> Result<Json<i32>, Status> {
    let mut manager = db.deref().get().await;
    let tx = manager.as_mut().get_transaction().await?;
    let user_dao = UsersDao::from(&tx);

    let None = user_dao.find_by_username(&login_dto.username).await.map_err(Into::into)? else {
        return Err(Status::BadRequest);
    };

    let Ok(password) = bcrypt::hash_with_salt(&login_dto.password, 10, Uuid::new_v4().into_bytes())
        .as_ref()
        .map(ToString::to_string) else {
            return Err(Status::InternalServerError);
        };

    let login_dto = UserLoginDto { password, ..login_dto.0 };

    let user_info = user_dao.create(&login_dto).await.map_err(Into::into)?;

    tx.commit().await.map_err(Into::into)?;

    Ok(Json(user_info.id))
}

#[get("/user")]
pub async fn display_info(user: Login, db: &State<Pool>) -> Result<Json<UserInfoDto>, Status> {
    let mut manager = db.deref().get().await;
    let tx = manager.as_mut().get_transaction().await?;
    let user_dao = UsersDao::from(&tx);

    let Some(info) = user_dao.find_display_info_by_id(user.id).await.map_err(Into::into)? else {
        return Err(Status::BadRequest)
    };

    Ok(Json(info))
}

#[delete("/user")]
pub async fn delete_user(user: Login, db: &State<Pool>, cookie_jar: &CookieJar<'_>) -> Result<(), Status> {
    let mut manager = db.deref().get().await;
    let tx = manager.as_mut().get_transaction().await?;
    let user_dao = UsersDao::from(&tx);

    user_dao.delete(user.id).await.map_err(Into::into)?;

    cookie_jar.get("auth").map(|c| cookie_jar.remove(c.clone()));

    Ok(())
}