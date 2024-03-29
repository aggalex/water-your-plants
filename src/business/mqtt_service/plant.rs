use crate::business::cdi::transaction::TransactionContext;
use crate::business::cdi::Injects;
use crate::business::manager::plant_manager::PlantManger;
use crate::business::manager::ErrorResponse;
use crate::business::mqtt_service::{ClientDelegate};
use crate::db;

use crate::persistence::Transaction;
use client::MeasurementDTO;
use monadic_mqtt::mqtt::event::{SubscribeEvent};
use rocket::futures::TryFutureExt;
use rocket::http::hyper::body::HttpBody;


use std::future::Future;


pub async fn new_measurement(
    uuid: &str,
    measurement_dto: &MeasurementDTO,
) -> Result<(), ErrorResponse> {
    let mut manager = db::get().await.get().await?;
    let context = TransactionContext::new(Transaction::new(&mut manager).await?);

    let plant_manager: PlantManger = context.inject();
    plant_manager.update_moisture(uuid, measurement_dto).await?;

    context.commit(()).await
}

impl SubscribeEvent for ClientDelegate<MeasurementDTO> {
    type Error = ();

    fn invoke(&self) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send {
        new_measurement(&self.uuid, &self.event).map_err(|e| {
            eprintln!("   >> mqtt error: {:?}", e);
        })
    }
}
