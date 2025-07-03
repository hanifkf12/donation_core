use crate::infrastructure::database::connection::establish_connection_pool;
use crate::infrastructure::web::routes;
use actix_web::middleware::Logger;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use dotenvy::dotenv;

mod application;
mod domain;
mod infrastructure;

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = establish_connection_pool()
        .await
        .expect("Failed to establish connection pool");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(ping)
            .configure(routes::routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
