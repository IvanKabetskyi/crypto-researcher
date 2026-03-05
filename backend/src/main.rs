mod application;
mod domain;
mod infrastructure;

use actix_cors::Cors;
use actix_web::{App, HttpServer};

use application::controller::history::get_history;
use application::controller::analyze::trigger_analysis;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port: u16 = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".into())
        .parse()
        .expect("SERVER_PORT must be a valid u16");

    infrastructure::services::scheduler::start_scheduler(
        std::env::var("ANALYSIS_INTERVAL_SECS")
            .unwrap_or_else(|_| "300".into())
            .parse()
            .expect("ANALYSIS_INTERVAL_SECS must be a valid u64"),
    );

    tracing::info!("Starting server on {}:{}", host, port);

    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(get_history)
            .service(trigger_analysis)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
