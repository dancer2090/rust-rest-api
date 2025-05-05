use actix_web::{web, HttpResponse, Responder, Result};
use validator::Validate;
use crate::scopes::user::types::{CreateUserBody, UserParams, UpdateUserBody};
use crate::scopes::user::service as UserService;
use crate::errors;


pub async fn get_users() -> Result <impl Responder> {
  Ok(web::Json(UserService::get_all_users()))
}

pub async fn get_user(path: web::Path<UserParams>) -> Result <impl Responder, actix_web::Error > {
  let params = path.into_inner();
  
  if let Err(e) = params.validate() {
    return Ok(HttpResponse::BadRequest().json(e));
  }

  let user = UserService::get_all_users().into_iter().find(|user| user.id == params.id);

  match user {
    Some(user) => Ok(HttpResponse::Ok().json(user)),
    None => Ok(HttpResponse::NotFound().json(errors::ResponseError {error: "User not found".to_string()})),
  }
}

pub async fn create_user(body: web::Json<CreateUserBody>) -> Result <impl Responder, actix_web::Error > {

  if let Err(e) = body.validate() {
    return Ok(HttpResponse::BadRequest().json(e));
  }

  let body = body.into_inner();

  let user = UserService::add_user(body.username, body.email);

  match user {
      Ok(user) => Ok(HttpResponse::Ok().json(user)),
      Err(e) => Ok(HttpResponse::BadRequest().json(errors::ResponseError { error: e })),
  }
}

pub async fn update_user(path: web::Path<UserParams>, body: web::Json<UpdateUserBody>) -> Result <impl Responder, actix_web::Error > {
  if let Err(e) = body.validate() {
    return Ok(HttpResponse::BadRequest().json(e));
  }
  let body = body.into_inner();
  let params = path.into_inner();
  let user = UserService::edit_user(params.id, body.username, body.email);
  match user {
    Ok(user) => Ok(HttpResponse::Ok().json(user)),
    Err(e) => Ok(HttpResponse::BadRequest().json(errors::ResponseError { error: e })),
  }
}

pub async fn delete_user(path: web::Path<UserParams>) -> Result <impl Responder> {
  let params = path.into_inner();
  UserService::remove_user(params.id);
  Ok(HttpResponse::Ok().finish()) 
}
