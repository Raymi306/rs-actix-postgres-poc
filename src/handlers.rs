use actix_web::{web, Error as ActixError, HttpResponse};
use deadpool_postgres::{Client, Pool};

use crate::{db, errors::Error as AppError, models::Account, models::NewAccountJSON};

pub async fn add_account(
    account: web::Json<NewAccountJSON>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, ActixError> {
    let account_info: NewAccountJSON = account.into_inner();
    let client: Client = db_pool.get().await.map_err(AppError::PoolError)?;
    let account = Account::new(account_info);
    let new_account_id: i32 = db::add_user(&client, account).await?;
    Ok(HttpResponse::Ok().json(new_account_id))
}
