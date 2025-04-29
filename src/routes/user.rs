use actix_web::{web};
use crate::middlewares;
use crate::scopes::user::handlers::{
  get_users,
  get_user,
  create_user,
  update_user,
  delete_user,
};
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
          .wrap(middlewares::auth::AuthMiddleware)
          .route("", web::get().to(get_users))
          .route("/{id}", web::get().to(get_user))
          .route("", web::post().to(create_user))
          .route("/{id}", web::put().to(update_user))
          .route("/{id}", web::delete().to(delete_user))
    );
}