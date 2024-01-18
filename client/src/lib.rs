extern crate monadic_mqtt;
extern crate serde;

use monadic_mqtt::mqtt::event::PublishEvent;
use serde::{Deserialize, Serialize};

#[cfg(feature = "bin")]
pub mod service;
#[cfg(feature = "bin")]
pub mod context;
#[cfg(feature = "bin")]
mod convert;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Error {
    HardwareError
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MeasurementDTO {
    Error(Error),
    Measurement {
        moisture: f32,
        temperature: f32,
        humidity: f32
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WaterRequestDTO {
    pub uuid: String,
    pub duration: i32,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UpdateCredentialsDTO {
    pub token: String
}

impl PublishEvent for MeasurementDTO {
    type Response = ();
    const TOPIC: &'static str = "/plant/measurement";
}

impl PublishEvent for WaterRequestDTO {
    type Response = ();
    const TOPIC: &'static str = "/plant/water";
}

impl PublishEvent for UpdateCredentialsDTO {
    type Response = ();
    const TOPIC: &'static str = "/auth/update";
}