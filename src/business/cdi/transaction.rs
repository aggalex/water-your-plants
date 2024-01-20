use std::future::Future;
use std::ops::Deref;
use deadpool::managed::Object;
use deadpool_postgres::{Manager, Pool};
use rocket::http::hyper::body::HttpBody;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::request::FromRequest;
use crate::business::cdi::{DefaultContext, Injects};
use crate::business::manager::ErrorResponse;
use crate::persistence::{Error, Transaction};

pub struct TransactionContext<'a> {
    transaction: Transaction<'a>
}

impl<'a> TransactionContext<'a> {
    pub fn new(transaction: Transaction<'a>) -> TransactionContext<'a> {
        TransactionContext { transaction }
    }


    pub async fn commit<R>(self, value: R) -> Result<R, ErrorResponse> {
        self.transaction.commit().await?;
        Ok(value)
    }
}

impl<'a> Into<DefaultContext> for TransactionContext<'a> {
    fn into(self) -> DefaultContext {
        DefaultContext
    }
}

impl<'a> Injects<'a, &'a Transaction<'a>> for TransactionContext<'a> {
    fn inject(&'a self) -> &'a Transaction<'a> {
        &self.transaction
    }
}

impl<'a> TransactionContext<'a> {
    pub async fn commit_transaction(self) -> Result<(), Error> {
        self.transaction.commit().await
    }
}