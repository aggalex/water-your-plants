#![cfg(feature = "bin")]

extern crate dht11;
extern crate dotenv;
extern crate lazy_static;
extern crate monadic_mqtt;
extern crate rppal;
extern crate serde;
extern crate tokio;

use client::WaterRequestDTO;
use dotenv::dotenv;
use monadic_mqtt::mqtt::Listener;
use rumqttc::v5::MqttOptions;
use std::time::Duration;
use tokio::join;
use client::service::measurement::measurement_service;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let connection = client::context::connection::get();

    let mut mqttoptions = MqttOptions::new(
        client::context::uuid().await,
        &connection.mqtt_server.host().expect("MQTT Server is missing host").to_string(),
        connection.mqtt_server.port().unwrap_or(1883),
    );
    mqttoptions.set_credentials(&connection.mqtt_username, &connection.mqtt_password);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let mut listener = Listener::new(mqttoptions, 10);

    let con = listener.connection().clone();
    let water_listener = tokio::spawn(async move {
        listener.subscribe::<WaterRequestDTO>().await.listen().await;
    });

    println!("Launching measurement service");

    let measurement_service = tokio::spawn(async move {
        measurement_service(con).await;
    });

    join!(water_listener, measurement_service);

    println!("measurement service exited");

}
