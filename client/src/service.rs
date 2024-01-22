use crate::context::hardware;
use crate::{context, ClientEvent, Error, MeasurementDTO, WaterRequestDTO};
use monadic_mqtt::mqtt::event::{PublishEvent, SubscribeEvent};
use rppal::hal::Delay;
use rppal::{gpio, spi};
use std::future::Future;
use std::time::Duration;
impl From<gpio::Error> for Error {
    fn from(_: gpio::Error) -> Self {
        Error::HardwareError
    }
}

impl From<spi::Error> for Error {
    fn from(_: spi::Error) -> Self {
        Error::HardwareError
    }
}

impl<E> From<dht11::Error<E>> for Error {
    fn from(_: dht11::Error<E>) -> Self {
        Error::HardwareError
    }
}

impl SubscribeEvent for WaterRequestDTO {
    type Error = Error;

    fn invoke(&self) -> impl Future<Output = Result<Self::Response, Self::Error>> {
        let uuid = self.uuid.clone();
        async move {
            if uuid != context::uuid().await {
                return Ok(());
            }

            let hardware = context::hardware::get();
            let mut valve = hardware.solenoid_valve()?;

            if self.set_active {
                valve.set_high();
            } else {
                valve.set_low();
            }

            Ok(())
        }
    }
}

// #[cfg(not(feature = "mock"))]
// pub mod measurement {
//
//     use super::*;
//
//     pub fn measure() -> Result<MeasurementDTO, Error> {
//         let hardware = hardware::get();
//         let mut dht = hardware.dht11()?;
//         let moisture_percentage = hardware.moisture_sensor_get_normalized_value()?;
//         let reading = dht.perform_measurement(&mut Delay::new())?;
//         Ok(MeasurementDTO::Measurement {
//             moisture: moisture_percentage,
//             temperature: reading.temperature as f32 / 10.0,
//             humidity: reading.humidity as f32 / 10.0,
//         })
//     }
//
//     pub async fn measurement_service(conn: monadic_mqtt::mqtt::Connection) {
//         loop {
//             ClientEvent::from(measure().unwrap_or_else(|e| MeasurementDTO::Error(e)))
//                 .await
//                 .publish(conn.clone())
//                 .await
//                 .unwrap_or_else(|e| eprintln!("Unable to publish measurement: {:?}", e));
//             tokio::time::sleep(Duration::from_secs(1)).await;
//         }
//     }
// }

pub mod measurement {
    use std::sync::Arc;
    use lazy_static::lazy_static;
    use tokio::sync::{Mutex, MutexGuard};
    use super::*;

    pub async fn get<'a>() -> MutexGuard<'a, MeasurementDTO> {
        lazy_static! {
            static ref ENVIRONMENT: Arc<Mutex<MeasurementDTO>> = Arc::new(Mutex::new(MeasurementDTO::Measurement {
                moisture: 40.0,
                temperature: 25.0,
                humidity: 60.0,
            }));
        }

        ENVIRONMENT.lock().await
    }

    pub fn measure(measurement_dto: &MeasurementDTO) -> MeasurementDTO {
        match &measurement_dto {
            MeasurementDTO::Error(e) => MeasurementDTO::Error(e.clone()),
            MeasurementDTO::Measurement { moisture, temperature, humidity } => {
                let diff = moisture * 0.1;

                let hardware = hardware::get();
                let Ok(mut valve) = hardware.solenoid_valve() else {
                    return MeasurementDTO::Error(Error::HardwareError)
                };

                let moisture = if valve.is_set_high() { moisture + 10.0 } else { moisture - diff };

                MeasurementDTO::Measurement {
                    moisture,
                    temperature: temperature.clone(),
                    humidity: humidity.clone()
                }
            }
        }
    }


    pub async fn measurement_service(conn: monadic_mqtt::mqtt::Connection) {
        loop {
            let mut measurement = get().await;
            *measurement = measure(&*measurement);
            ClientEvent::from(measurement.clone()).await
                .publish(conn.clone())
                .await
                .unwrap_or_else(|e| eprintln!("Unable to publish measurement: {:?}", e));
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}