use crate::business::cdi::transaction::TransactionContext;
use crate::business::cdi::Injects;
use crate::business::http_service::middleware::login::LoggedInUserDTO;
use crate::business::manager::ErrorResponse;
use crate::persistence::entity::login::{LoginDTO, LoginDao};
use crate::persistence::entity::user::{UserInfoDto, UserLoginDto, UsersDao};
use crate::persistence::{Error, Transaction};
use rocket::http::{Cookie, Status};
use rocket::response::status::BadRequest;
use uuid::Uuid;

pub struct UserManager<'a> {
    pub user_dao: UsersDao<'a>,
    pub login_dao: LoginDao<'a>,
}

impl<'a> Injects<'a, UserManager<'a>> for TransactionContext<'a> {
    fn inject(&'a self) -> UserManager<'a> {
        UserManager {
            user_dao: self.inject(),
            login_dao: self.inject(),
        }
    }
}

impl UserManager<'_> {
    pub async fn login(
        &self,
        login_dto: UserLoginDto,
        auth_key: Option<String>,
    ) -> Result<LoginDTO, ErrorResponse> {
        if !auth_key.is_none() {
            return Err(ErrorResponse::BadRequest(
                "You are already logged in".to_string(),
            ));
        };

        let Some(user) = self.user_dao.find_by_username(&login_dto.username).await? else {
            return Err(ErrorResponse::BadRequest("Invalid credentials".to_string()));
        };

        let Ok(true) = bcrypt::verify(&login_dto.password, &user.password) else {
            return Err(ErrorResponse::BadRequest("Invalid credentials".to_string()));
        };

        let login = self.login_dao.login(user.id).await?;

        Ok(login)
    }

    pub async fn logout(&self, user: LoggedInUserDTO) -> Result<(), ErrorResponse> {
        self.login_dao.logout(&user.key).await.map_err(Into::into)
    }

    pub async fn register(&self, login_dto: UserLoginDto) -> Result<UserInfoDto, ErrorResponse> {
        let None = self.user_dao.find_by_username(&login_dto.username).await? else {
            return Err(ErrorResponse::BadRequest("User already exists".to_string()));
        };

        let password = bcrypt::hash_with_salt(&login_dto.password, 10, Uuid::new_v4().into_bytes())
            .as_ref()?
            .to_string();

        let login_dto = UserLoginDto {
            password,
            ..login_dto
        };

        let user_info = self.user_dao.create(&login_dto).await?;

        Ok(UserInfoDto {
            id: user_info.id,
            username: user_info.username,
        })
    }

    pub async fn get_user_info(&self, user: LoggedInUserDTO) -> Result<UserInfoDto, ErrorResponse> {
        let Some(info) = self.user_dao.find_display_info_by_id(user.id).await? else {
            return Err(ErrorResponse::BadRequest("No such user".to_string()));
        };

        Ok(info)
    }

    pub async fn delete(&self, user: LoggedInUserDTO) -> Result<(), ErrorResponse> {
        self.user_dao.delete(user.id).await.map_err(Into::into)
    }

    pub async fn clear_logins(&self) -> Result<(), ErrorResponse> {
        self.login_dao.clear().await?;
        Ok(())
    }
}
