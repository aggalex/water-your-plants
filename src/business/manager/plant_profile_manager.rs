use crate::business::cdi::transaction::TransactionContext;
use crate::business::cdi::Injects;
use crate::business::dto::PaginationDto;
use crate::business::manager::ErrorResponse;
use crate::persistence::dao::plant_profile::{
    NewPlantProfileDto, PlantProfileDao, PlantProfileDto,
};


pub struct PlantProfileManager<'a> {
    plant_profile_dao: PlantProfileDao<'a>,
}

impl<'a> Injects<'a, PlantProfileManager<'a>> for TransactionContext<'a> {
    fn inject(&'a self) -> PlantProfileManager<'a> {
        PlantProfileManager {
            plant_profile_dao: self.inject(),
        }
    }
}

impl PlantProfileManager<'_> {
    pub async fn get_profiles(
        &self,
        query: Option<String>,
        pagination_dto: PaginationDto,
    ) -> Result<Vec<PlantProfileDto>, ErrorResponse> {
        let profiles = self
            .plant_profile_dao
            .search(query.as_ref().map(|str| &str[..]), &pagination_dto)
            .await?;
        Ok(profiles)
    }

    pub async fn create_profile(
        &self,
        new_plant_profile_dto: NewPlantProfileDto,
    ) -> Result<PlantProfileDto, ErrorResponse> {
        let profile = self
            .plant_profile_dao
            .create(&new_plant_profile_dto)
            .await?;
        Ok(profile)
    }
}
