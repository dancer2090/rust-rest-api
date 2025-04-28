use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/{user_id}")]
async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("User Profile")
}

#[post("/")]
async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("New User")
}

#[put("/")]
async fn update_user() -> impl Responder {
    HttpResponse::Ok().body("User Update")
}

#[delete("/")]
async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("User Delete")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user)
       .service(create_user)
       .service(update_user)
       .service(delete_user);
}