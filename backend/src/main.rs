mod application;
mod domain;
mod infrastructure;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::from_fn};

use application::controller::analyze::analyze_stream;
use application::controller::auth::login;
use application::controller::config::get_config;
use application::controller::history::get_history;
use application::middleware::auth_middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port: u16 = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".into())
        .parse()
        .expect("SERVER_PORT must be a valid u16");

    // Seed default users
    application::usecases::seed_users::seed_users().await;

    infrastructure::services::scheduler::start_scheduler(
        std::env::var("ANALYSIS_INTERVAL_SECS")
            .unwrap_or_else(|_| "300".into())
            .parse()
            .expect("ANALYSIS_INTERVAL_SECS must be a valid u64"),
    );

    tracing::info!("Starting server on {}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .wrap(from_fn(auth_middleware))
            .service(login)
            .service(get_config)
            .service(get_history)
            .service(analyze_stream)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
