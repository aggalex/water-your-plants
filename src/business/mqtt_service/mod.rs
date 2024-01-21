mod plant;

use crate::business::cdi::{GlobalContext, Injects};
use crate::business::manager::ErrorResponse;
use client::{ClientEvent, MeasurementDTO};
use lazy_static::lazy_static;
use monadic_mqtt::mqtt::error::PublishError;
use monadic_mqtt::mqtt::event::{PublishEvent, SubscribeEvent};
use monadic_mqtt::mqtt::{Connection, Listener};
use rumqttc::v5::MqttOptions;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::OnceCell;

lazy_static! {
    static ref CONN: OnceCell<Arc<Connection>> = OnceCell::new();
}

pub struct Mqtt {
    pub listener: Listener,
}

impl Mqtt {
    pub fn new() -> Mqtt {
        let mut mqttoptions = MqttOptions::new(
            "server",
            std::env::var("MQTT_BROKER_HOST").expect("No MQTT Broker host set up"),
            std::env::var("MQTT_BROKER_PORT")
                .expect("No MQTT Broker port set up")
                .parse()
                .expect("Malformed MQTT Broker port"),
        );
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        Mqtt {
            listener: Listener::new(mqttoptions, 10),
        }
    }

    pub async fn listen(&mut self) {
        CONN.set(Arc::from(self.listener.connection().clone())).ok();

        self.listener
            .subscribe::<ClientDelegate<MeasurementDTO>>()
            .await
            .listen()
            .await
    }
}

impl Injects<'_, Connection> for GlobalContext {
    fn inject(&self) -> Connection {
        CONN.get().unwrap().deref().clone()
    }
}

impl From<PublishError> for ErrorResponse {
    fn from(value: PublishError) -> Self {
        eprintln!("Publish error: {value:?}");
        ErrorResponse::InternalServerError(())
    }
}

pub(self) struct Delegate<T>(T);

impl<T: Serialize> Serialize for Delegate<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'d, T: Deserialize<'d>> Deserialize<'d> for Delegate<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'d>,
    {
        T::deserialize(deserializer).map(Delegate)
    }
}

impl<T: PublishEvent> PublishEvent for Delegate<T> {
    type Response = T::Response;
    const TOPIC: &'static str = T::TOPIC;
}

impl<T> Delegate<T> {
    pub fn into_owned(self) -> T {
        self.0
    }

    pub fn new(value: T) -> Self {
        Delegate(value)
    }
}

impl<T> Deref for Delegate<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Delegate<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub(self) type ClientDelegate<T> = Delegate<ClientEvent<T>>;
