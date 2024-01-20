use deadpool_postgres::Pool;
use crate::business::cdi::Injects;
use crate::business::cdi::transaction::TransactionContext;
use crate::business::manager::ErrorResponse;
use crate::business::manager::user_manager::UserManager;
use crate::persistence::entity::login::LoginDao;
use crate::persistence::{Error, Transaction};

pub async fn dump(db: Pool) {

    let result: Result<(), ErrorResponse> = try {
        let mut manager = db.get().await?;
        let mut context = TransactionContext::new(Transaction::new(&mut manager).await?);

        let user_manager: UserManager = context.inject();

        user_manager.clear_logins().await?;
    };

    if let Err(err) = result {
        eprintln!("   >> DB Error in dumps: {:?}", err)
    }
}

