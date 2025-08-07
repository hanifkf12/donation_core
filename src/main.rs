use crate::infrastructure::database::connection::establish_connection_pool;
use crate::infrastructure::telemetry::otel::init_tracer;
use crate::infrastructure::web::routes;
use actix_web::middleware::Logger;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use dotenvy::dotenv;
use opentelemetry::global::shutdown_tracer_provider;
use tokio::signal::unix::{signal, SignalKind};
use std::sync::Arc;
use tokio::sync::Notify;

mod application;
mod domain;
mod infrastructure;

use actix_web_opentelemetry::RequestTracing;

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let pool = establish_connection_pool()
        .await
        .expect("Failed to establish connection pool");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let _ = init_tracer();

    // Create a notify instance for graceful shutdown
    let shutdown = Arc::new(Notify::new());
    let shutdown_clone = shutdown.clone();

    // Set up signal handlers
    let _signal_handler = tokio::spawn(async move {
        let mut sigterm = signal(SignalKind::terminate()).expect("Failed to register SIGTERM handler");
        let mut sigint = signal(SignalKind::interrupt()).expect("Failed to register SIGINT handler");
        
        tokio::select! {
            _ = sigterm.recv() => {
                log::info!("Received SIGTERM, starting graceful shutdown");
            }
            _ = sigint.recv() => {
                log::info!("Received SIGINT, starting graceful shutdown");
            }
        }
        
        shutdown_clone.notify_one();
    });

    // Start the HTTP server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(RequestTracing::new())
            .app_data(web::Data::new(pool.clone()))
            .service(ping)
            .configure(routes::routes)
    })
    .bind(("127.0.0.1", 9090))?
    .run();

    // Create a handle to the server for graceful shutdown
    let server_handle = server.handle();
    
    // Spawn the server task
    let server_task = tokio::spawn(server);
    
    // Wait for shutdown signal
    shutdown.notified().await;
    log::info!("Starting graceful shutdown process");
    
    // Gracefully stop the server
    server_handle.stop(true).await;
    
    // Wait for the server to complete
    let result = server_task.await.expect("Server task panicked")?;
    
    // Clean up resources
    log::info!("Shutting down OpenTelemetry tracer");
    shutdown_tracer_provider();
    
    log::info!("Graceful shutdown completed");
    Ok(result)
}
