use rocket::http::Status;

use crate::persistence::Error;

pub mod cdi;
pub mod r#data;
pub mod dto;
pub mod http_service;
pub mod manager;
pub mod mqtt_service;
