# Rust PoC web server

This proof of concept uses [actix-web](https://actix.rs/) and [tokio\_postgres](https://docs.rs/tokio-postgres/latest/tokio_postgres/) to serve a simple HTTP REST API.
It is based off of [this](https://github.com/actix/examples/tree/master/databases/postgres) example.
The proof of concept is exploring what the Rust workflow feels like given a common developing task of writing and running a server backend, and to share the results with coworkers.

## Extra libraries:
- [argon2](https://docs.rs/argon2/latest/argon2/)
- [config](https://docs.rs/config/latest/config/)
- [chrono](https://docs.rs/chrono/latest/chrono/)

## Instructions
1. Create database user

   ```shell
   createuser -P testing
   ```

   Enter a password of your choice. The following instructions assume you used `testing` as password.

   This step is **optional** and you can also use an existing database user for that. Just make sure to replace `test_user` by the database user of your choice in the following steps and change the `.env` file containing the configuration accordingly.

   An alternative using SQL:
   ```sql
   CREATE USER testing WITH PASSWORD 'testing';
   ```

2. Create database

   ```shell
   createdb -O testing testing_db
   ```

   An alternative using SQL:
   ```sql
   CREATE DATABASE testing_db OWNER test_user;
   ```

3. Initialize database

   ```shell
   psql -f sql/schema.sql testing_db
   ```

   This step can be repeated and clears the database as it drops and recreates the schema `testing` which is used within the database.

4. Grant privileges to new user

   ```sql
   GRANT ALL PRIVILEGES ON SCHEMA testing TO testing;
   GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA testing TO testing;
   GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA testing TO testing;
   ``` 

5. Create `.env` file:

   ```ini
   SERVER_ADDR=127.0.0.1:8080
   PG.USER=testing
   PG.PASSWORD=testing
   PG.HOST=127.0.0.1
   PG.PORT=5432
   PG.DBNAME=testing_db
   PG.POOL.MAX_SIZE=16
   ```

6. Run the server:

   ```shell
   cargo run
   ```

7. Using a different terminal send an HTTP POST request to the running server:

   ```shell
   curl -i -d '{"email": "ferris@thecrab.com", "full_name": "ferris crab", "user_name": "ferreal", "password_raw": "CORRECT HORSE BATTERY STAPLE"}' -H 'Content-Type: application/json' http://127.0.0.1:8080/users
   ```
