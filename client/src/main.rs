#![cfg(feature = "bin")]

extern crate dht11;
extern crate dotenv;
extern crate lazy_static;
extern crate monadic_mqtt;
extern crate rppal;
extern crate serde;
extern crate tokio;

use client::service::measurement_service;
use client::WaterRequestDTO;
use dotenv::dotenv;
use monadic_mqtt::mqtt::Listener;
use rumqttc::v5::MqttOptions;
use std::time::Duration;

#[tokio::main]
async fn main() {
    dotenv().ok();

    client::context::fetch_uuid().await.unwrap();
    let connection = client::context::connection::get();

    let mut mqttoptions = MqttOptions::new(
        &client::context::uuid().await,
        &connection.server.to_string(),
        connection.mqtt_port,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let mut listener = Listener::new(mqttoptions, 10);

    let con = listener.connection().clone();
    tokio::spawn(measurement_service(con));

    listener.subscribe::<WaterRequestDTO>().await.listen().await;
}
