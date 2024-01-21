use rocket::http::Status;

use crate::persistence::Error;

pub mod http_service;
pub mod r#data;
pub mod mqtt_service;
pub mod manager;
pub mod cdi;
pub mod dto;
