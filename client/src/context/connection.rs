use crate::context::{num_var, var};
use lazy_static::lazy_static;
use std::error::Error;
use std::fmt::{Debug, Display, format, Formatter};
use std::sync::Arc;
use url::{ParseError, Url};

pub struct Connection {
    pub http_server: Url,
    pub mqtt_server: Url,
}

pub fn get() -> Arc<Connection> {
    lazy_static! {
        static ref CONNECTION: Arc<Connection> = Arc::new(Connection::new());
    }
    return CONNECTION.clone();
}

impl Connection {
    fn new() -> Self {
        Self {
            http_server: Url::parse(&var("CONN_HTTP_SERVER")).expect("Malformed HTTP Server URL"),
            mqtt_server: Url::parse(&var("CONN_MQTT_SERVER")).expect("Malformed MQTT Server URL"),
        }
    }
}