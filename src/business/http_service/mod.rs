use deadpool_postgres::Pool;
use monadic_mqtt::mqtt::Connection;
use rocket::{Rocket, routes};
use rocket::request::FromRequest;
use crate::business::cdi::DefaultContext;
use crate::persistence::Transaction;

mod user;
pub mod middleware;
mod plant;

pub async fn http(db: Pool, mqtt: Connection) {
    Rocket::build()
        .manage(db)
        .manage(mqtt)
        .mount("/user", user::routes())
        .launch().await.unwrap();
}