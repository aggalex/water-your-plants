use crate::business::cdi::transaction::TransactionContext;
use crate::business::cdi::Injects;
use crate::persistence::entity::login::LoginDao;
use crate::persistence::Transaction;
use deadpool_postgres::Pool;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, State, ___internal_try_outcome as try_outcome};

#[derive(From, Clone)]
pub struct LoggedInUserDTO {
    pub id: i32,
    pub key: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for LoggedInUserDTO {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let res: Result<LoggedInUserDTO, Status> = try {
            let cookie = request
                .cookies()
                .get_private("auth")
                .ok_or(Status::Unauthorized)?;

            let db = try_outcome!(request.guard::<&State<Pool>>().await);
            let mut manager = db.get().await.map_err(|_| Status::InternalServerError)?;
            let mut context = TransactionContext::new(
                Transaction::new(&mut manager)
                    .await
                    .map_err(|_| Status::InternalServerError)?,
            );
            let tx: &Transaction = context.inject();

            LoginDao::from(tx)
                .get_user_id_of_key(cookie.value())
                .await
                .map_err(|_| Status::InternalServerError)?
                .map(|id| LoggedInUserDTO {
                    id,
                    key: cookie.value().to_string(),
                })
                .ok_or(Status::Unauthorized)?
        };

        match res {
            Ok(login) => Outcome::Success(login),
            Err(status) => Outcome::Error((status, ())),
        }
    }
}
