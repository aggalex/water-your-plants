use deadpool_postgres::{CreatePoolError, ManagerConfig, Pool, RecyclingMethod};
use lazy_static::lazy_static;
use tokio_postgres::NoTls;
use crate::business;

lazy_static! {
    static ref DB: tokio::sync::OnceCell<Pool> = tokio::sync::OnceCell::new();
}

pub async fn connect() -> Result<Pool, CreatePoolError> {
    deadpool_postgres::Config {
        host: Some("localhost".to_string()),
        user: Some("postgres".to_string()),
        password: Some("12341234".to_string()),
        dbname: Some("irrigate".to_string()),
        manager: Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast
        }),
        ..deadpool_postgres::Config::new()
    }.create_pool(None, NoTls)
}

async fn init() -> Pool {
    let db = connect().await.unwrap();

    tokio::spawn({
        let db = db.clone();
        business::r#data::dump(db)
    });

    return db.clone();
}

pub async fn get() -> &'static Pool {
    DB.get_or_init(init).await
}