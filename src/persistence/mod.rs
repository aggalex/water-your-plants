pub mod entity;
mod query_builder;


use std::ops::{Deref};
use deadpool_postgres::{CreatePoolError, GenericClient, Manager, ManagerConfig, Pool, PoolError, RecyclingMethod};
use postgres_from_row::FromRow;
use tokio_postgres::{NoTls, Row};

pub async fn connect() -> Result<Pool, CreatePoolError> {
    deadpool_postgres::Config {
        host: Some("localhost".to_string()),
        user: Some("postgres".to_string()),
        password: Some("12341234".to_string()),
        dbname: Some("irrigate".to_string()),
        manager: Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast
        }),
        ..deadpool_postgres::Config::new()
    }.create_pool(None, NoTls)
}

#[derive(Debug, From)]
pub enum Error {
    Pool(PoolError),
    Postgres(tokio_postgres::Error)
}

pub struct Transaction<'a> (deadpool_postgres::Transaction<'a>);

impl<'a> Transaction<'a> {
    pub async fn new(manager: &'a mut deadpool::managed::Object<Manager>) -> Result<Self, Error> {
        manager.transaction().await.map(Transaction).map_err(Into::into)
    }

    pub async fn commit(self) -> Result<(), Error> {
        self.0.commit().await.map_err(Into::into)
    }
}

impl<'a> Deref for Transaction<'a> {
    type Target = deadpool_postgres::Transaction<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

type QueryResult<T> = std::result::Result<T, Error>;

pub(self) trait FromRowExtension<T: FromRow> {
    fn try_from_opt_row(opt: Option<Row>) -> Result<Option<T>, Error>;
    fn try_from_row_owned(row: Row) -> Result<T, Error>;
}

impl<T: FromRow> FromRowExtension<T> for T {
    fn try_from_opt_row(opt: Option<Row>) -> Result<Option<T>, Error> {
        opt.as_ref().map(<T as FromRow>::try_from_row).transpose().map_err(Into::into)
    }
    fn try_from_row_owned(row: Row) -> Result<T, Error> {
        T::try_from_row(&row).map_err(Into::into)
    }
}