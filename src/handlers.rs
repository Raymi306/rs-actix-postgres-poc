use actix_web::{web, Error as ActixError, HttpResponse};
use argon2::{
    password_hash::{
        PasswordHash,
        PasswordVerifier,
    },
    Argon2
};
use deadpool_postgres::{Client, Pool};
use jsonwebtoken::{
    encode,
    Algorithm,
    EncodingKey,
    Header,
};

use crate::{
    db,
    errors::Error as AppError,
    models::Account,
    models::NewAccountJSON,
    models::LoginInfo,
    models::LoginResponse,
    models::Claims,
};

pub async fn add_account(
    account: web::Json<NewAccountJSON>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, ActixError> {
    let account_info: NewAccountJSON = account.into_inner();
    let client: Client = db_pool.get().await.map_err(AppError::PoolError)?;
    let account = Account::new(account_info);
    let new_account_id: i64 = db::add_user(&client, account).await?;
    Ok(HttpResponse::Ok().json(new_account_id))
}

pub async fn get_accounts(db_pool: web::Data<Pool>) -> Result<HttpResponse, ActixError> {
    let client = db_pool.get().await.map_err(AppError::PoolError)?;
    let accounts = db::get_users(&client).await?;
    Ok(HttpResponse::Ok().json(accounts))
}

pub async fn login(
    login_info: web::Json<LoginInfo>,
    db_pool: web::Data<Pool>
) -> Result<HttpResponse, ActixError> {
    let secret = b"I WOULD EXIST IN THE CONFIG NOT HERE";
    let client = db_pool.get().await.map_err(AppError::PoolError)?;
    let user = db::get_user(&client, &login_info.user_name).await?;
    let parsed_hash = PasswordHash::new(&user.password).unwrap();
    if !Argon2::default()
        .verify_password(login_info.password_raw.as_bytes(), &parsed_hash)
        .is_ok() {
            return Ok(HttpResponse::Unauthorized().finish());
    }
    let claims = Claims {
        sub: user.account_id.unwrap().to_string()
    };
    let header = Header::new(Algorithm::HS512);
    let token = encode(&header, &claims, &EncodingKey::from_secret(secret)).unwrap();
    Ok(HttpResponse::Ok().json(LoginResponse { token }))
}
