use deadpool_postgres::Pool;
use crate::business::DbExtensions;
use crate::persistence::entity::login::LoginDao;
use crate::persistence::{Error, Transaction};

pub async fn dump(db: Pool) {
    let mut manager = db.get().await;
    let Ok(tx) = manager.as_mut().get_transaction().await else {
        eprintln!("   >> Failed to launch transaction");
        return;
    };

    let mut errors = vec![];

    clear_logins(&tx).await.unwrap_or_else(|err| errors.push(err));

    for error in errors {
        eprintln!("   >> DB Error in dumps: {:?}", error)
    }
}

pub async fn clear_logins(tx: &Transaction<'_>) -> Result<(), Error> {
    let login_dao = LoginDao::from(tx);
    login_dao.clear().await?;
    Ok(())
}