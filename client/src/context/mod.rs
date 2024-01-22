use lazy_static::lazy_static;
use std::error::Error;
use tokio::sync::{OnceCell, RwLock};

pub mod connection;
pub mod hardware;

pub async fn uuid() -> &'static str {
    lazy_static! {
        static ref UUID: OnceCell<String> = OnceCell::new();
    }

    UUID.get_or_init(|| async { var("PLANT_UUID") }).await
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
