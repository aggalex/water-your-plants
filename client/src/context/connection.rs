use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use lazy_static::lazy_static;
use reqwest::{Client, Method, Response, Url};
use url::ParseError;
use crate::context::{num_var, var};

pub struct Connection {
    pub server: Url,
    pub http_port: u16,
    pub mqtt_port: u16,
    pub auth_token: String
}

pub fn get() -> Arc<Connection> {
    lazy_static! {
        static ref CONNECTION: Arc<Connection> = Arc::new(Connection::new());
    }
    return CONNECTION.clone()
}

impl Connection {

    fn new() -> Self {
        Self {
            server: Url::parse(&var("CONN_SERVER")).expect("Malformed Server URL"),
            http_port: num_var("CONN_HTTP_PORT"),
            mqtt_port: num_var("CONN_MQTT_PORT"),
            auth_token: var("CONN_AUTH_TOKEN")
        }
    }

    pub async fn request(&self, method: Method, path: &str) -> Result<Response, RequestError> {
        let client = Client::new();
        let request = client.request(method, self.server.join(path)?)
            .header("client_auth", &self.auth_token)
            .build()?;
        Ok(client.execute(request).await?)
    }
}

#[derive(Debug)]
pub enum RequestError {
    Url(url::ParseError),
    Request(reqwest::Error)
}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::Url(err) => write!(f, "{err}"),
            RequestError::Request(err) => write!(f, "{err}"),
        }
    }
}

impl Error for RequestError {

}

impl From<reqwest::Error> for RequestError {
    fn from(value: reqwest::Error) -> Self {
        RequestError::Request(value)
    }
}

impl From<url::ParseError> for RequestError {
    fn from(value: ParseError) -> Self {
        RequestError::Url(value)
    }
}