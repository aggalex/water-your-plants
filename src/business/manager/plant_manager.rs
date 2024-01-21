use crate::business::cdi::transaction::TransactionContext;
use crate::business::cdi::{GlobalContext, Injects};
use crate::business::dto::PaginationDto;
use crate::business::http_service::middleware::login::LoggedInUserDTO;

use crate::business::manager::ErrorResponse;
use crate::persistence::entity::plant::{NewPlantDto, PlantDao, PlantDto};
use crate::persistence::entity::plant_profile::PlantProfileDao;

use client::{MeasurementDTO, WaterRequestDTO};
use monadic_mqtt::mqtt::event::PublishEvent;
use monadic_mqtt::mqtt::Connection;
use rocket::futures::TryFutureExt;

pub struct PlantManger<'a> {
    plant_dao: PlantDao<'a>,
    plant_profile_dao: PlantProfileDao<'a>,
    conn: Connection,
}

impl<'a> Injects<'a, PlantManger<'a>> for TransactionContext<'a> {
    fn inject(&'a self) -> PlantManger<'a> {
        let global: GlobalContext = self.inject();
        PlantManger {
            plant_dao: self.inject(),
            plant_profile_dao: self.inject(),
            conn: global.inject(),
        }
    }
}

impl PlantManger<'_> {
    pub async fn update_moisture(
        &self,
        uuid: &str,
        measurement_dto: &MeasurementDTO,
    ) -> Result<(), ErrorResponse> {
        match measurement_dto {
            MeasurementDTO::Error(err) => {
                eprintln!("   >> Client Error: {:?}", err);
                return Ok(());
            }
            MeasurementDTO::Measurement { moisture, .. } => {
                let Some(plant) = self
                    .plant_dao
                    .update_moisture(uuid, moisture.clone())
                    .await?
                else {
                    return Ok(());
                };
                self.check_for_watering(plant).await?;
            }
        }

        Ok(())
    }

    pub async fn check_for_watering(&self, plant_dto: PlantDto) -> Result<(), ErrorResponse> {
        println!("Checking plant {:?}", plant_dto);
        let profile = self
            .plant_profile_dao
            .find_by_id(plant_dto.profile_id)
            .await?
            .ok_or(ErrorResponse::InternalServerError(()))?;

        println!("Profile {:?}", profile);

        if plant_dto.soil_moisture < profile.max_moisture
            && plant_dto.soil_moisture > profile.min_moisture
        {
            println!("ok");
            return Ok(());
        }

        println!("Water request");

        WaterRequestDTO {
            uuid: plant_dto.uuid.clone(),
            set_active: plant_dto.soil_moisture <= profile.min_moisture,
        }
        .publish(self.conn.clone())
        .await
        .map_err(Into::into)
    }

    pub async fn create_plant(
        &self,
        new_plant_dto: NewPlantDto,
        logged_in_user_dto: LoggedInUserDTO,
    ) -> Result<PlantDto, ErrorResponse> {
        let plant = self
            .plant_dao
            .create(new_plant_dto, logged_in_user_dto.id)
            .await?;

        Ok(plant)
    }

    pub async fn get_plants_of_user(
        &self,
        query: Option<String>,
        pagination: PaginationDto,
        logged_in_user_dto: LoggedInUserDTO,
    ) -> Result<Vec<PlantDto>, ErrorResponse> {
        let plant = self
            .plant_dao
            .search_by_belongs_to_user_id(
                query.as_ref().map(|s| &**s),
                logged_in_user_dto.id,
                pagination,
            )
            .await?;

        Ok(plant)
    }

    pub async fn assert_user_owns_plant(
        &self,
        id: i32,
        logged_in_user_dto: &LoggedInUserDTO,
    ) -> Result<(), ErrorResponse> {
        let owner_id = self.plant_dao.get_belongs_to_user_id(id).await?;

        if owner_id.filter(|id| *id == logged_in_user_dto.id).is_none() {
            return Err(ErrorResponse::Forbidden(()));
        }

        Ok(())
    }

    pub async fn delete_plant(
        &self,
        id: i32,
        logged_in_user_dto: LoggedInUserDTO,
    ) -> Result<(), ErrorResponse> {
        self.assert_user_owns_plant(id, &logged_in_user_dto).await?;

        self.plant_dao.delete(id).await?;

        Ok(())
    }

    pub async fn update_plant(
        &self,
        id: i32,
        new_plant_dto: NewPlantDto,
        logged_in_user_dto: LoggedInUserDTO,
    ) -> Result<(), ErrorResponse> {
        self.assert_user_owns_plant(id, &logged_in_user_dto).await?;

        self.plant_dao.update(id, new_plant_dto).await?;

        Ok(())
    }
}
