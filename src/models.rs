use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, Serialize)]
pub struct NewAccountJSON {
    pub user_name: String,
    pub email: String,
    pub full_name: String,
    pub password_raw: String,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "account")]
pub struct Account {
    pub account_id: Option<i64>,
    pub created_at: Option<DateTime<Utc>>,
    pub modified_at: Option<DateTime<Utc>>,
    pub user_name: String,
    pub email: String,
    pub full_name: String,
    pub password: String,
}

impl Account {
    pub fn new(account_info: NewAccountJSON) -> Self {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password = argon2.hash_password(account_info.password_raw.as_bytes(), &salt).unwrap().to_string();
        Self {
            account_id: None,
            created_at: None,
            modified_at: None,
            user_name: account_info.user_name,
            email: account_info.email,
            full_name: account_info.full_name,
            password
        }
    }
}

