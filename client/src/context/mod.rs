use lazy_static::lazy_static;
use reqwest::Method;
use std::error::Error;
use tokio::sync::RwLock;

pub mod connection;
pub mod hardware;

lazy_static! {
    static ref UUID: RwLock<String> = RwLock::new("".to_string());
}

pub async fn uuid() -> String {
    UUID.read().await.clone()
}

pub async fn set_uuid(uuid: String) {
    *UUID.write().await = uuid;
}

pub async fn fetch_uuid() -> Result<(), Box<dyn Error>> {
    let response = connection::get()
        .request(Method::POST, "/node/register")
        .await?;
    set_uuid(serde_json::from_str(&String::from_utf8(
        response.bytes().await?.to_vec(),
    )?)?)
    .await;
    Ok(())
}

pub(self) fn num_var<X: TryFrom<u64>>(var_name: &str) -> X {
    var(var_name)
        .parse::<u64>()
        .ok()
        .and_then(|number| X::try_from(number).ok())
        .expect(&format!("{var_name} value is invalid"))
}

pub(self) fn var(var_name: &str) -> String {
    std::env::var(var_name).expect(&format!("{var_name} is missing"))
}
