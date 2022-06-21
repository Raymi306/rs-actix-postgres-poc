use crate::{errors::Error, models::Account};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn add_user(client: &Client, user_info: Account) -> Result<i32, Error> {
    let _stmt = include_str!("../sql/add_user.sql");
    let _stmt = _stmt.replace("$table_fields", &Account::sql_table_fields());
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
        .or_else(|_| Err(Error::NotFound)) // more applicable for SELECTs
        .unwrap()
        .get("account_id");
    Ok(result)
}
