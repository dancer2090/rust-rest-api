use actix_web::{web, HttpResponse, Responder, Result};
use serde_json;
use uuid::Uuid;
use std::fs::File;
use std::io::Write;
use crate::scopes::user::types::{User, UserBody, UserParams,};
use crate::errors;

fn add_user(username: String, email: String) -> Result<User, String> {
  let uuid = Uuid::new_v4();
  
  let user = User {
    id: uuid,
    email,
    username
  };

  add_to_file(user.clone())?;
  
  Ok(user)
}

fn add_to_file(user: User) -> Result<(), String> {
  let mut users = get_all_users();

  let user_exists = match users.iter().find(|u| u.email == user.email) {
    Some(user) => true,
    None => false,
  };

  users.push(user);

  match user_exists {
      false => {
        let list_as_json = serde_json::to_string(&users).unwrap();
        let mut file = File::create("users.json").expect("Cant create users.json");
        file.write_all(list_as_json.as_bytes()).expect("Cannot write to the file!");
      },
      true => return Err("User already exists".to_string()),
  };
  Ok(())
}

fn get_all_users() -> Vec<User> {
    let file = File::open("users.json").expect("File users.json not found");
    let users: Vec<User> = serde_json::from_reader(file).expect("Fail to read users.json");
    users
}

pub async fn get_users() -> Result <impl Responder> {
  Ok(web::Json(get_all_users()))
}

pub async fn get_user(path: web::Path<UserParams>) -> impl Responder {
  let params = path.into_inner();
  println!("{}", params.id);

  let user = get_all_users().into_iter().find(|user| user.id == params.id);
  
  let error = errors::ResponseError {
    error: "User not found".to_string()
  };

  match user {
    Some(user) => HttpResponse::Ok().json(user),
    None => HttpResponse::NotFound().json(error),
  }
}

pub async fn create_user(body: web::Json<UserBody>) -> Result <impl Responder> {
  let body = body.into_inner();

  let user = add_user(body.username, body.email);

  match user {
      Ok(user) => Ok(HttpResponse::Ok().json(user)),
      Err(e) => Err(actix_web::error::ErrorBadRequest(e )),
  }
  // match user {
  //     Ok(user) => HttpResponse::Ok().json(user),
  //     Err(e) => HttpResponse::NotFound().json(errors::ResponseError { error: e }),
  // }
}

pub async fn update_user() -> Result <impl Responder> {
  let user: User = get_all_users()[0].clone();
  Ok(web::Json(user))
}

pub async fn delete_user() -> Result <impl Responder> {
  Ok(HttpResponse::Ok().finish())    
}
