use deadpool_postgres::Pool;
use rocket::request::{FromRequest, Outcome};
use rocket::{___internal_try_outcome as try_outcome, Request, State};
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use crate::business::DbExtensions;
use crate::persistence::entity::login::LoginDao;

#[derive(From, Clone)]
pub struct Login {
    pub id: i32,
    pub key: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Login {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookie = try_outcome!(request.cookies()
            .get_private("auth")
            .or_error((Status::Unauthorized, ())));

        let db = try_outcome!(request.guard::<&State<Pool>>().await);
        let mut binding = db.get().await;
        let manager = binding.as_mut();
        let tx = try_outcome!(manager.get_transaction().await
            .map_err(|_| ()).or_error(Status::InternalServerError));

        let res = try_outcome!(LoginDao::from(&tx).get_user_id_of_key(cookie.value()).await
            .map_err(|_| ())
            .or_error(Status::InternalServerError))
            .map(|id| Login { id, key: cookie.value().to_string() }).or_error((Status::Unauthorized, ()));

        res
    }
}
