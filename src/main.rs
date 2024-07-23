use actix_web::{HttpServer, App, middleware};
use rust_project::server::Server;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    println!("Starting server on http://127.0.0.1:8080");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::default())
            .configure(Server::configure_services)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
