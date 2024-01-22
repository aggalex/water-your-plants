extern crate monadic_mqtt;
extern crate serde;

use monadic_mqtt::mqtt::event::PublishEvent;
use serde::{Deserialize, Serialize};

#[cfg(feature = "bin")]
pub mod context;
#[cfg(feature = "bin")]
mod convert;
#[cfg(feature = "bin")]
pub mod service;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Error {
    HardwareError,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MeasurementDTO {
    Error(Error),
    Measurement {
        moisture: f32,
        temperature: f32,
        humidity: f32,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientEvent<T: Serialize> {
    pub uuid: String,
    pub event: T,
}

#[cfg(feature = "bin")]
impl<T: Serialize> ClientEvent<T> {
    pub async fn from(value: T) -> ClientEvent<T> {
        use crate::context::uuid;

        ClientEvent {
            uuid: uuid().await.to_string(),
            event: value,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WaterRequestDTO {
    pub uuid: String,
    pub set_active: bool,
}

impl PublishEvent for ClientEvent<MeasurementDTO> {
    type Response = ();
    const TOPIC: &'static str = "/plant/measurement";
}

impl PublishEvent for WaterRequestDTO {
    type Response = ();
    const TOPIC: &'static str = "/plant/water";
}
