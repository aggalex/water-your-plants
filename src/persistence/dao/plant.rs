use crate::business::cdi::transaction::TransactionContext;
use crate::business::cdi::Injects;
use crate::business::dto::PaginationDto;

use crate::persistence::{FromRowExtension, FromRowsExtension, QueryResult, Transaction};
use chrono::{DateTime, Utc};
use postgres_from_row::FromRow;
use rocket::serde::{Deserialize, Serialize};
use tokio_postgres::types::ToSql;
use uuid::Uuid;

#[derive(Clone, FromRow, Serialize, Debug)]
pub struct PlantDto {
    pub id: i32,
    pub name: String,
    pub profile_id: i32,
    pub last_watered: Option<DateTime<Utc>>,
    pub uuid: String,
    pub belongs_to_user_id: i32,
    pub soil_moisture: f32,
}


#[derive(Deserialize, Clone, FromRow)]
pub struct NewPlantDto {
    pub name: String,
    pub profile_id: i32,
}

#[derive(From, Clone)]
pub struct PlantDao<'r>(&'r Transaction<'r>);

impl<'r> Injects<'r, PlantDao<'r>> for TransactionContext<'r> {
    fn inject(&'r self) -> PlantDao<'r> {
        PlantDao(self.inject())
    }
}

impl PlantDao<'_> {
    pub async fn find_by_id(&self, id: i32) -> QueryResult<Option<PlantDto>> {
        self.0
            .query_opt(r#"SELECT * FROM plant WHERE id = $1"#, &[&id])
            .await
            .map_err(Into::into)
            .and_then(PlantDto::try_from_opt_row)
    }

    pub async fn search_by_belongs_to_user_id(
        &self,
        query: Option<&str>,
        user_id: i32,
        pagination: PaginationDto,
    ) -> QueryResult<Vec<PlantDto>> {
        let conditional_clause = r#"AND "name" like $3"#;

        let offset = pagination.offset();
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![&pagination.page_size, &offset, &user_id];

        let query = query.map(|query| format!("%{query}%"));

        if let Some(query) = query.as_ref() {
            params.push(query)
        }

        self.0
            .query(
                &format!(
                    r#"SELECT * FROM plant WHERE belongs_to_user_id = $3 {} LIMIT $1 OFFSET $2"#,
                    query.as_ref().map(|_| conditional_clause).unwrap_or("")
                ),
                &params[..],
            )
            .await
            .map_err(Into::into)
            .and_then(Vec::<PlantDto>::try_collect)
    }

    pub async fn get_belongs_to_user_id(&self, plant_id: i32) -> QueryResult<Option<i32>> {
        #[derive(FromRow)]
        struct PlantPartDTO {
            belongs_to_user_id: i32,
        };

        let opt = self
            .0
            .query_opt(
                r#"SELECT belongs_to_user_id from plant where id = $1"#,
                &[&plant_id],
            )
            .await?;
        let opt = PlantPartDTO::try_from_opt_row(opt)?;
        Ok(opt.map(|plant| plant.belongs_to_user_id))
    }

    pub async fn update_moisture(
        &self,
        uuid: &str,
        moisture: f32,
    ) -> QueryResult<Option<PlantDto>> {
        self.0
            .query_opt(
                r#"UPDATE plant SET soil_moisture = $2 WHERE uuid = $1 RETURNING *"#,
                &[&uuid, &moisture],
            )
            .await
            .map_err(Into::into)
            .and_then(PlantDto::try_from_opt_row)
    }

    pub async fn create(&self, plant_dto: NewPlantDto, user_id: i32) -> QueryResult<PlantDto> {
        self.0.query_one(r#"INSERT INTO plant("name", profile_id, uuid, belongs_to_user_id) VALUES ($1, $2, $3, $4) RETURNING *"#, &[
            &plant_dto.name,
            &plant_dto.profile_id,
            &Uuid::new_v4().to_string(),
            &user_id
        ]).await.map_err(Into::into).and_then(PlantDto::try_from_row_owned)
    }

    pub async fn update(
        &self,
        id: i32,
        new_plant_dto: NewPlantDto,
    ) -> QueryResult<Option<PlantDto>> {
        self.0
            .query_opt(
                r#"UPDATE plant SET name = $2, profile_id = $3 WHERE id = $1 RETURNING *"#,
                &[&id, &new_plant_dto.name, &new_plant_dto.profile_id],
            )
            .await
            .map_err(Into::into)
            .and_then(PlantDto::try_from_opt_row)
    }

    pub async fn delete(&self, id: i32) -> QueryResult<()> {
        self.0
            .query(r#"DELETE FROM plant WHERE id = $1"#, &[&id])
            .await
            .map_err(Into::into)
            .map(|_| ())
    }
}
