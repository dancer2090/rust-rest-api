use actix_web::{web, HttpResponse, Responder};

pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("User Profile")
}

pub async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("User Profile")
}

pub async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("New User")
}

pub async fn update_user() -> impl Responder {
    HttpResponse::Ok().body("User Update")
}

pub async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("User Delete")
}
