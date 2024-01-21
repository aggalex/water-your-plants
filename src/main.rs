#![feature(try_blocks)]
#![feature(associated_type_defaults)]

pub mod business;
pub mod db;
pub mod persistence;

// extern crate dothyphen;
// extern crate futures;
// extern crate tokio;
// extern crate tokio_stream;
// extern crate tokio_util;
extern crate monadic_mqtt;
extern crate serde_json;
extern crate tokio;
extern crate tokio_stream;
#[macro_use]
extern crate derive_more;

use crate::business::http_service::http;
use crate::business::mqtt_service::Mqtt;


#[rocket::main]
async fn main() {
    dotenv::dotenv().unwrap();
    let _db = db::get().await.clone();
    let mut mqtt = Mqtt::new();
    let http_handle = tokio::spawn(http(
        db::get().await.clone(),
        mqtt.listener.connection().clone(),
    ));
    let mqtt_handle = tokio::spawn(async move { mqtt.listen().await });
    tokio::join!(http_handle, mqtt_handle);
}
