pub mod entity;
use deadpool::managed::Object;
use deadpool_postgres::{GenericClient, Manager, PoolError};
use postgres_from_row::FromRow;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use tokio_postgres::Row;

#[derive(Debug, From)]
pub enum Error {
    Pool(PoolError),
    Postgres(tokio_postgres::Error),
    Sync,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Pool(err) => write!(f, "{err}"),
            Error::Postgres(err) => write!(f, "{err}"),
            Error::Sync => write!(f, "synchronization error"),
        }
    }
}

pub struct Transaction<'a>(deadpool_postgres::Transaction<'a>);

impl<'a> Transaction<'a> {
    pub async fn new(manager: &'a mut Object<Manager>) -> Result<Self, Error> {
        manager
            .transaction()
            .await
            .map(Transaction)
            .map_err(Into::into)
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
        opt.as_ref()
            .map(<T as FromRow>::try_from_row)
            .transpose()
            .map_err(Into::into)
    }

    fn try_from_row_owned(row: Row) -> Result<T, Error> {
        T::try_from_row(&row).map_err(Into::into)
    }
}

pub(self) trait FromRowsExtension<T: FromRow, I: FromIterator<T>> {
    fn try_collect(iter: impl IntoIterator<Item = Row>) -> Result<I, Error>;
}

impl<T: FromRow, I: FromIterator<T>> FromRowsExtension<T, I> for I {
    fn try_collect(iter: impl IntoIterator<Item = Row>) -> Result<I, Error> {
        iter.into_iter()
            .map(T::try_from_row_owned)
            .collect::<Result<I, Error>>()
    }
}
