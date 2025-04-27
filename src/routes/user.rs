use actix_web::{get, web, HttpResponse, Responder};

#[get("/profile")]
async fn profile() -> impl Responder {
    HttpResponse::Ok().body("User Profile")
}

#[get("/settings")]
async fn settings() -> impl Responder {
    HttpResponse::Ok().body("User Settings")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(profile)
       .service(settings);
}