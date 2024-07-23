use actix_web::web;
use crate::controllers::user_controller::UserController;
use crate::services::user_service::UserService;

pub fn init_user_routes(cfg: &mut web::ServiceConfig) {
    let service = web::Data::new(UserService::new());

    cfg.app_data(service.clone())
        .service(
            web::scope("/users")
                .route("", web::get().to(UserController::get_users))
                .route("/{id}", web::delete().to(UserController::delete_user))
                .route("/ws", web::get().to(UserController::chat_route))
        );
}
