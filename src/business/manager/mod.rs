use crate::persistence::Error;
use bcrypt::BcryptError;
use deadpool::managed::PoolError;
use rocket::http::Status;
use rocket::Responder;

pub mod plant_manager;
pub mod plant_profile_manager;
pub mod user_manager;

#[derive(Responder, Debug)]
pub enum ErrorResponse {
    #[response(status = 400, content_type = "json")]
    BadRequest(String),
    #[response(status = 500)]
    InternalServerError(()),
    #[response(status = 403)]
    Forbidden(()),
}

impl From<Error> for ErrorResponse {
    fn from(value: Error) -> Self {
        eprintln!("   >> DB Error: {}", value);
        ErrorResponse::InternalServerError(())
    }
}

impl From<PoolError<tokio_postgres::Error>> for ErrorResponse {
    fn from(value: PoolError<tokio_postgres::Error>) -> Self {
        ErrorResponse::from(Error::from(value))
    }
}

impl<'a> From<&'a BcryptError> for ErrorResponse {
    fn from(value: &'a BcryptError) -> Self {
        eprintln!("   >> Bcrypt Error: {:?}", value);
        ErrorResponse::InternalServerError(())
    }
}
