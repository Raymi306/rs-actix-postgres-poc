DROP SCHEMA IF EXISTS testing CASCADE;
CREATE SCHEMA testing;

CREATE TABLE IF NOT EXISTS testing.account (
	account_id bigserial PRIMARY KEY,
	created_at timestamp DEFAULT current_timestamp,
	modified_at timestamp DEFAULT current_timestamp,
	user_name varchar NOT NULL UNIQUE,
	email varchar(254) NOT NULL UNIQUE,
	full_name varchar NOT NULL,
	password varchar NOT NULL
);
