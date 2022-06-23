mod config;
mod db;
mod errors;
mod guards;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer, guard::fn_guard};
use guards::authorization_guard;
use handlers::add_account;
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = crate::config::ExampleConfig::new();
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/users")
                     .route(web::post().to(add_account))
                     .guard(fn_guard(authorization_guard))
                     )
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
