use chrono::{DateTime, Utc};
use postgres_from_row::FromRow;
use uuid::Uuid;
use crate::persistence::{FromRowExtension, QueryResult, Transaction};


#[derive(Clone, FromRow)]
pub struct PlantDto {
    pub id: i32,
    pub name: String,
    pub profile_id: i32,
    pub last_watered: Option<DateTime<Utc>>,
    pub uuid: String,
    pub soil_moisture: f32,
}

#[derive(Clone, FromRow)]
pub struct NewPlantDto {
    pub name: String,
    pub profile_id: i32,
}

#[derive(From, Clone)]
pub struct PlantDao<'r>(&'r Transaction<'r>);

impl PlantDao<'_> {
    pub async fn find_by_id(&self, id: i32) -> QueryResult<Option<PlantDto>> {
        self.0.query_opt(r#"SELECT * FROM plant WHERE id = $1"#, &[&id])
            .await.map_err(Into::into).and_then(PlantDto::try_from_opt_row)
    }

    pub async fn update_moisture(&self, id: i32, moisture: f32) -> QueryResult<Option<PlantDto>> {
        self.0.query_opt(r#"UPDATE plant SET soil_moisture = $2 WHERE id = $1 RETURNING *"#, &[&id, &moisture])
            .await.map_err(Into::into).and_then(PlantDto::try_from_opt_row)
    }

    pub async fn create(&self, plant_dto: NewPlantDto) -> QueryResult<PlantDto> {
        self.0.query_one(r#"INSERT INTO plant("name", profile_id, uuid) VALUES ($1, $2, $3) RETURNING *"#, &[
            &plant_dto.name,
            &plant_dto.profile_id,
            &Uuid::new_v4().to_string()
        ]).await.map_err(Into::into).and_then(PlantDto::try_from_row_owned)
    }

    pub async fn update(&self, id: i32, new_plant_dto: NewPlantDto) -> QueryResult<Option<PlantDto>> {
        self.0.query_opt(r#"UPDATE plant SET name = $2, profile_id = $3 WHERE id = $1 RETURNING *"#, &[
            &id,
            &new_plant_dto.name,
            &new_plant_dto.profile_id
        ]).await.map_err(Into::into).and_then(PlantDto::try_from_opt_row)
    }

    pub async fn delete(&self, id: i32) -> QueryResult<()> {
        self.0.query(r#"DELETE FROM plant WHERE id = $1"#, &[&id])
            .await.map_err(Into::into).map(|_| ())
    }
}