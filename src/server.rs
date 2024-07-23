use actix_web::{web::{self, ServiceConfig}, HttpResponse, Responder, Error};
use serde_json::json;
use crate::routers::{user_router::init_user_routes, product_router::init_product_routes};

pub struct Server;

impl Server {

    pub fn configure_services(cfg: &mut ServiceConfig) {
        cfg.service(
            web::scope("/api")
                .configure(init_user_routes)
                .configure(init_product_routes)
        )
        .default_service(web::route().to(Server::not_found))
        .app_data(web::Data::new(Server::handle_errors));
    }

    async fn not_found() -> impl Responder {
        let response_body = json!({
            "error": "Not Found",
            "message": "The requested resource was not found on this server."
        });

        HttpResponse::NotFound()
            .content_type("application/json")
            .body(response_body.to_string())
    }


    async fn handle_errors(error: Error) -> impl Responder {
        let status_code = error.as_response_error().status_code();
        let response_body = json!({
            "message": error.to_string(),
            "status_code": status_code.as_u16()
        });

        HttpResponse::build(status_code)
            .content_type("application/json")
            .body(response_body.to_string())
    }
}
