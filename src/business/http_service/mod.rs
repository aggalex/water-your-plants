

use deadpool_postgres::Pool;
use monadic_mqtt::mqtt::Connection;

use rocket::{Rocket};
use rocket_cors::CorsOptions;

pub mod middleware;
mod plant;
mod plant_profile;
mod user;

pub async fn http(db: Pool, mqtt: Connection) {
    Rocket::build()
        .manage(db)
        .manage(mqtt)
        .mount("/user", user::routes())
        .mount("/plant", plant::routes())
        .mount("/profile", plant_profile::routes())
        .attach(rocket_cors::CorsOptions {
            allow_credentials: true,
            ..CorsOptions::default()
        }.to_cors().unwrap())
        .launch()
        .await
        .unwrap();
}
