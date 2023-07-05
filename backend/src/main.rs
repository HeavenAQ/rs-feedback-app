use std::env;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use backend::app_state::AppState;
use backend::handlers;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await;
    let pool = match pool {
        Ok(pool) => {
            println!("Database connection has be established.");
            pool
        }
        Err(err) => {
            eprint!("Failed to connect to database\n{}", err);
            std::process::exit(1);
        }
    };
    println!("Server has started successfully!");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_methods(vec!["POST", "PATCH", "GET", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ]);
        App::new()
            .app_data(web::Data::new(AppState {
                db_pool: pool.clone(),
            }))
            .configure(handlers::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
