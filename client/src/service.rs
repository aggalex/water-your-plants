use std::future::Future;
use std::time::Duration;
use monadic_mqtt::mqtt::event::{PublishEvent, SubscribeEvent};
use rppal::{gpio, spi};
use rppal::hal::Delay;
use crate::{ClientEvent, context, Error, MeasurementDTO, WaterRequestDTO};
use crate::context::hardware;
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
        let duration = Duration::from_secs(self.duration as u64);
        async move {
            if uuid != context::uuid().await {
                return Ok(())
            }

            let hardware = context::hardware::get();
            let mut valve = hardware.solenoid_valve()?;

            valve.set_high();
            tokio::time::sleep(duration).await;
            valve.set_low();

            Ok(())
        }
    }
}

pub fn measure() -> Result<MeasurementDTO, Error> {
    let hardware = hardware::get();
    let mut dht = hardware.dht11()?;
    let moisture_percentage = hardware.moisture_sensor_get_normalized_value()?;
    let reading = dht.perform_measurement(&mut Delay::new())?;
    Ok(MeasurementDTO::Measurement {
        moisture: moisture_percentage,
        temperature: reading.temperature as f32 / 10.0,
        humidity: reading.humidity as f32 / 10.0,
    })
}

pub async fn measurement_service(conn: monadic_mqtt::mqtt::Connection) {
    loop {
        ClientEvent::from(measure().unwrap_or_else(|e| MeasurementDTO::Error(e))).await
            .publish(conn.clone()).await
            .unwrap_or_else(|e| eprintln!("Unable to publish measurement: {:?}", e));
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}