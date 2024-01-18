use deadpool_postgres::Pool;
use rocket::{Rocket, routes};

mod user;

pub async fn http(db: Pool) {
    Rocket::build()
        .manage(db)
        .mount("/", routes![
            user::login,
            user::logout,
            user::register,
            user::display_info,
            user::delete_user
        ])
        .launch().await.unwrap();
}
