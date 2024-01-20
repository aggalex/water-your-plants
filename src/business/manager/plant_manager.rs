use client::MeasurementDTO;
use crate::business::cdi::Injects;
use crate::business::cdi::transaction::TransactionContext;
use crate::business::manager::ErrorResponse;
use crate::persistence::entity::plant::PlantDao;
use crate::persistence::Transaction;

pub struct PlantManger<'a> {
    plant_dao: PlantDao<'a>
}

impl<'a> Injects<'a, PlantManger<'a>> for TransactionContext<'a> {
    fn inject(&'a self) -> PlantManger<'a> {
        let transaction: &Transaction = self.inject();
        PlantManger {
            plant_dao: PlantDao::from(transaction),
        }
    }
}

impl PlantManger<'_> {
    pub async fn update_moisture(&self, uuid: &str, measurement_dto: &MeasurementDTO) -> Result<(), ErrorResponse> {
        match measurement_dto {
            MeasurementDTO::Error(err) => {
                eprintln!("   >> Client Error: {:?}", err);
                return Ok(())
            }
            MeasurementDTO::Measurement { moisture, .. } => {
                self.plant_dao.update_moisture(uuid, moisture.clone()).await?;
            }
        }
        Ok(())
    }
}