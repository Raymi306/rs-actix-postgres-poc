use crate::{errors::Error, models::Account};
use deadpool_postgres::Client;
use tokio_postgres::error::SqlState;

pub async fn add_user(client: &Client, user_info: Account) -> Result<i64, Error> {
    let _stmt = include_str!("../sql/add_user.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();

    let result = client
        .query_one(
            &stmt,
            &[
                &user_info.user_name,
                &user_info.email,
                &user_info.full_name,
                &user_info.password,
            ],
        )
        .await
        .map_err(|error| {
            if *error.code().unwrap() == SqlState::UNIQUE_VIOLATION {
                return Error::DuplicateEntry
            } else {
                return Error::PGError(error)
            }
        })?
        .get("account_id");
    return Ok(result)
}
