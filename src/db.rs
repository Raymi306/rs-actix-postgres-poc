use deadpool_postgres::Client;
use tokio_postgres::error::SqlState;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{errors::Error, models::Account};

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

pub async fn get_users(client: &Client) -> Result<Vec<Account>, Error> {
    let _stmt = include_str!("../sql/get_users.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();

    let result = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Account::from_row_ref(row).unwrap())
        .collect::<Vec<Account>>();

    return Ok(result)
}

pub async fn get_user(client: &Client, user_name: &String) -> Result<Account, Error> {
    let _stmt = include_str!("../sql/get_user.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();

    let result = client
        .query_one(&stmt, &[user_name])
        .await?;
    Ok(Account::from_row_ref(&result).unwrap())
}
