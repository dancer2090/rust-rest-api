use actix_web::{web, HttpResponse, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct User {
  username: String,
  email: String,
}

struct Users {
  users: [User]
}

pub async fn get_users() -> Result <impl Responder> {

  let users = vec![
    User {
        username: "ivan".into(),
        email: "ivan@gmail.com".into(),
    },
    User {
        username: "maria".into(),
        email: "maria@example.com".into(),
    },
  ];

  Ok(web::Json(users))
}

pub async fn get_user() -> Result <impl Responder> {
    
  let obj = User {
    username: "ivan".to_string(),
    email: "ivan@gmail.com".into(),
  };
  Ok(web::Json(obj))
}

pub async fn create_user() -> Result <impl Responder> {
  let obj = User {
    username: "ivan".to_string(),
    email: "ivan@gmail.com".into(),
  };
  Ok(web::Json(obj))
}

pub async fn update_user() -> Result <impl Responder> {
  let obj = User {
    username: "ivan".to_string(),
    email: "ivan@gmail.com".into(),
  };
  Ok(web::Json(obj))
}

pub async fn delete_user() -> impl Responder {
    HttpResponse::Ok().finish()
}
