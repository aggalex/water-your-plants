use std::ops::{Deref, DerefMut};
use deadpool_postgres::Manager;
use rocket::http::Status;
use rocket::request::FromRequest;
use crate::persistence::{Error, Transaction};

pub mod service;
pub(self) mod middleware;
pub mod r#data;

pub(self) trait DbExtensions<'a> {
    async fn get_transaction(self) -> Result<Transaction<'a>, Status>;
}

impl<'a, E> DbExtensions<'a> for Result<&'a mut deadpool::managed::Object<Manager>, E> {
    async fn get_transaction(self) -> Result<Transaction<'a>, Status> {
        match self {
            Ok(manager) => Transaction::new(manager).await.map_err(|_| Status::InternalServerError),
            Err(_) => Err(Status::InternalServerError)
        }
    }
}

impl Into<Status> for Error {
    fn into(self) -> Status {
        eprintln!("   >> DB Error: {:?}", self);
        Status::InternalServerError
    }
}
