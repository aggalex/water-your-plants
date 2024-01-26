use crate::business::cdi::transaction::TransactionContext;
use crate::business::cdi::Injects;
use crate::business::dto::PaginationDto;

use crate::persistence::{FromRowExtension, FromRowsExtension, QueryResult, Transaction};
use deadpool_postgres::GenericClient;
use postgres_from_row::FromRow;
use rocket::serde::{Deserialize, Serialize};
use tokio_postgres::types::ToSql;

#[derive(Clone, FromRow, Serialize, Debug)]
pub struct PlantProfileDto {
    pub id: i32,
    pub name: String,
    pub max_moisture: f32,
    pub min_moisture: f32,
}

#[derive(Clone, Deserialize)]
pub struct NewPlantProfileDto {
    pub name: String,
    pub max_moisture: f32,
    pub min_moisture: f32,
}

#[derive(From, Clone)]
pub struct PlantProfileDao<'r>(&'r Transaction<'r>);

impl<'r> Injects<'r, PlantProfileDao<'r>> for TransactionContext<'r> {
    fn inject(&'r self) -> PlantProfileDao<'r> {
        PlantProfileDao(self.inject())
    }
}

impl PlantProfileDao<'_> {
    pub async fn search(
        &self,
        query: Option<&str>,
        pagination: &PaginationDto,
    ) -> QueryResult<Vec<PlantProfileDto>> {
        let conditional_clause = r#"AND "name" like $3"#;

        let offset = pagination.offset();
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![&pagination.page_size, &offset];

        let query = query.map(|query| format!("%{query}%"));

        if let Some(query) = query.as_ref() {
            params.push(query)
        }

        self.0
            .query(
                &format!(
                    r#"SELECT * FROM plant_profile {} LIMIT $1 OFFSET $2"#,
                    query.as_ref().map(|_| conditional_clause).unwrap_or("")
                ),
                &params[..],
            )
            .await
            .map_err(Into::into)
            .and_then(Vec::<PlantProfileDto>::try_collect)
    }

    pub async fn find_by_id(&self, id: i32) -> QueryResult<Option<PlantProfileDto>> {
        self.0
            .query_opt(r#"SELECT * FROM plant_profile WHERE id = $1"#, &[&id])
            .await
            .map_err(Into::into)
            .and_then(PlantProfileDto::try_from_opt_row)
    }

    pub async fn create(
        &self,
        new_plant_profile_dto: &NewPlantProfileDto,
    ) -> QueryResult<PlantProfileDto> {
        self.0.query_one(r#"INSERT INTO plant_profile (name, max_moisture, min_moisture) VALUES ($1, $2, $3) RETURNING *"#, &[
            &new_plant_profile_dto.name,
            &new_plant_profile_dto.max_moisture,
            &new_plant_profile_dto.min_moisture
        ]).await.map_err(Into::into).and_then(PlantProfileDto::try_from_row_owned)
    }

    pub async fn update(
        &self,
        id: i32,
        new_plant_profile_dto: &NewPlantProfileDto,
    ) -> QueryResult<PlantProfileDto> {
        self.0.query_one(r#"UPDATE plant_profile SET name = $2, max_moisture = $3, min_moisture = $4 WHERE id = $1 RETURNING *"#, &[
            &id,
            &new_plant_profile_dto.name,
            &new_plant_profile_dto.min_moisture,
            &new_plant_profile_dto.max_moisture
        ]).await.map_err(Into::into).and_then(PlantProfileDto::try_from_row_owned)
    }

    pub async fn delete(&self, id: i32) -> QueryResult<()> {
        self.0
            .query_one(r#"DELETE from plant_profile WHERE id = $1"#, &[&id])
            .await
            .map_err(Into::into)
            .map(|_| ())
    }
}
