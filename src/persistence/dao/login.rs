use chrono::{DateTime, Utc};
use deadpool_postgres::GenericClient;
use postgres_from_row::FromRow;

use crate::business::cdi::transaction::TransactionContext;
use crate::business::cdi::Injects;
use crate::persistence::{FromRowExtension, QueryResult, Transaction};
use uuid::Uuid;

#[derive(FromRow)]
pub struct LoginDTO {
    pub id: i32,
    pub at: DateTime<Utc>,
    pub user_id: i32,
    pub key: String,
}

#[derive(From, Clone)]
pub struct LoginDao<'a>(&'a Transaction<'a>);

impl<'a> Injects<'a, LoginDao<'a>> for TransactionContext<'a> {
    fn inject(&'a self) -> LoginDao<'a> {
        LoginDao(self.inject())
    }
}

impl LoginDao<'_> {
    pub async fn get_user_id_of_key(&self, key: &str) -> QueryResult<Option<i32>> {
        self.0
            .query_opt(r#"SELECT user_id from login where key = $1"#, &[&key])
            .await
            .map_err(Into::into)
            .map(|opt| opt.map(|row| row.get(0)))
    }

    pub async fn login(&self, user_id: i32) -> QueryResult<LoginDTO> {
        self.0
            .query_one(
                r#"INSERT INTO login (user_id, at,  key) VALUES ($1, $2, $3) RETURNING *"#,
                &[&user_id, &Utc::now(), &Uuid::new_v4().to_string()],
            )
            .await
            .map_err(Into::into)
            .and_then(LoginDTO::try_from_row_owned)
    }

    pub async fn logout(&self, key: &str) -> QueryResult<()> {
        self.0
            .query(r#"DELETE FROM login WHERE key = $1"#, &[&key])
            .await
            .map_err(Into::into)
            .map(|_| ())
    }

    pub async fn clear(&self) -> QueryResult<()> {
        self.0
            .query(r#"DELETE FROM login"#, &[])
            .await
            .map_err(Into::into)
            .map(|_| ())
    }
}
