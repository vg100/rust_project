use actix_web::{web, HttpResponse, Responder};
use crate::services::product_service::ProductService;
use crate::models::product_model::Product;

pub struct ProductController;

impl ProductController {
    pub async fn get_products(service: web::Data<ProductService>) -> impl Responder {
        let products = service.get_all_products();
        HttpResponse::Ok()
    }

    pub async fn create_product(service: web::Data<ProductService>, new_product: web::Json<Product>) -> impl Responder {
        let product = service.create_product(new_product.into_inner());
        HttpResponse::Created()
    }

    pub async fn update_product(service: web::Data<ProductService>, id: web::Path<u32>, updated_product: web::Json<Product>) -> impl Responder {
        let product = service.update_product(id.into_inner(), updated_product.into_inner());
        HttpResponse::Ok()
    }

    pub async fn delete_product(service: web::Data<ProductService>, id: web::Path<u32>) -> impl Responder {
        service.delete_product(id.into_inner());
        HttpResponse::NoContent().finish()
    }
}
