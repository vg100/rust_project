use actix_web::web;
use crate::controllers::product_controller::ProductController;
use crate::services::product_service::ProductService;

pub fn init_product_routes(cfg: &mut web::ServiceConfig) {
    let service = web::Data::new(ProductService::new());

    cfg.app_data(service.clone())
        .service(
            web::scope("/products")
                .route("", web::get().to(ProductController::get_products))
                .route("/{id}", web::delete().to(ProductController::delete_product))
        );
}
