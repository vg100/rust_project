use actix_web::{web, HttpResponse, Responder,HttpRequest};
use crate::services::user_service::UserService;
use crate::models::user_model::User;
use actix_web_actors::ws;

pub struct UserController;

impl UserController {
    pub async fn get_users(service: web::Data<UserService>) -> impl Responder {
        HttpResponse::Ok().body("User endpoint")
    }

    pub async fn create_user(service: web::Data<UserService>, new_user: web::Json<User>) -> impl Responder {
        let user = service.create_user(new_user.into_inner());
        HttpResponse::Created()
    }
    pub async fn update_user(service: web::Data<UserService>, id: web::Path<u32>, updated_user: web::Json<User>) -> impl Responder {
        let user = service.update_user(id.into_inner(), updated_user.into_inner());

        HttpResponse::Ok()
    }

    pub async fn delete_user(service: web::Data<UserService>, id: web::Path<u32>) -> impl Responder {
        service.delete_user(id.into_inner());
        HttpResponse::NoContent().finish()
    }

    pub async fn chat_route()-> impl Responder {
        HttpResponse::Ok().body("Test")
    }
}
