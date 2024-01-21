use deadpool_postgres::Pool;
use monadic_mqtt::mqtt::Connection;
use rocket::{Rocket, routes};
use rocket::request::FromRequest;
use crate::business::cdi::DefaultContext;
use crate::persistence::Transaction;

pub mod middleware;
mod user;
mod plant;
mod plant_profile;

pub async fn http(db: Pool, mqtt: Connection) {
    Rocket::build()
        .manage(db)
        .manage(mqtt)
        .mount("/user", user::routes())
        .mount("/plant", plant::routes())
        .mount("/profile", plant_profile::routes())
        .launch().await.unwrap();
}