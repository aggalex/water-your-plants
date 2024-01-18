#![feature(try_blocks)]
#![feature(associated_type_defaults)]

pub mod business;
pub mod persistence;

// extern crate dothyphen;
// extern crate futures;
// extern crate tokio;
// extern crate tokio_stream;
// extern crate tokio_util;
extern crate monadic_mqtt;
extern crate tokio;
extern crate tokio_stream;
extern crate serde_json;
#[macro_use]
extern crate derive_more;

use deadpool_postgres::Pool;
use rocket::{Rocket, routes, State};
use crate::business::service::http;
use crate::persistence::connect;

#[rocket::main]
async fn main() {
    let db = connect().await.unwrap();

    tokio::spawn({
        let db = db.clone();
        business::r#data::dump(db)
    });

    http(db).await;
}