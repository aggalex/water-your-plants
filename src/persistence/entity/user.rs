use chrono::{DateTime, Utc};
use deadpool_postgres::GenericClient;
use derive_more::From;
use postgres_from_row::FromRow;
use rocket::serde::{Deserialize, Serialize};
use crate::persistence::{FromRowExtension, QueryResult, Transaction};

#[derive(Deserialize)]
pub struct UserLoginDto {
    pub username: String,
    pub password: String
}

#[derive(FromRow, Serialize)]
pub struct FullUserInfoDto {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(FromRow, Serialize)]
pub struct UserInfoDto {
    pub id: i32,
    pub username: String
}

#[derive(From, Clone)]
pub struct UsersDao<'a>(&'a Transaction<'a>);

impl UsersDao<'_> {
    pub async fn find_by_id(&self, id: i32) -> QueryResult<Option<FullUserInfoDto>> {
        self.0.query_opt(r#"SELECT * FROM "user" WHERE id = $1"#, &[&id])
            .await.map_err(Into::into).and_then(FullUserInfoDto::try_from_opt_row)
    }

    pub async fn find_display_info_by_id(&self, id: i32) -> QueryResult<Option<UserInfoDto>> {
        self.0.query_opt(r#"SELECT id, username FROM "user" WHERE id = $1"#, &[&id])
            .await.map_err(Into::into).and_then(UserInfoDto::try_from_opt_row)
    }

    pub async fn find_by_username(&self, username: &str) -> QueryResult<Option<FullUserInfoDto>> {
        self.0.query_opt(r#"SELECT * FROM "user" WHERE username = $1"#, &[&username])
            .await.map_err(Into::into).and_then(FullUserInfoDto::try_from_opt_row)
    }

    pub async fn create_user(&self, user_register: &UserLoginDto) -> QueryResult<FullUserInfoDto> {
        self.0.query_one(r#"INSERT INTO "user"(username, password) VALUES ($1, $2) RETURNING *"#, &[
            &user_register.username,
            &user_register.password
        ]).await.map_err(Into::into).and_then(FullUserInfoDto::try_from_row_owned)
    }

    pub async fn delete_user(&self, id: i32) -> QueryResult<()> {
        self.0.query(r#"DELETE FROM "user" WHERE id = $1"#, &[&id])
            .await.map_err(Into::into).map(|_| ())
    }
}