use diesel::query_dsl::methods::{LimitDsl, SelectDsl};
use diesel::{SelectableHelper, RunQueryDsl};
use serde_json;
use uuid::Uuid;
use std::fs::File;
use std::io::Write;
use crate::scopes::user::types::User;
use crate::models;
use crate::establish_connection;


pub fn add_user(username: String, email: String) -> Result<User, String> {
  let uuid = Uuid::new_v4();
  
  let user = User {
    id: uuid,
    email,
    username
  };
  insert(user.clone())?;
  add_to_file(user.clone())?;
 
  Ok(user)
}

pub fn remove_user(id:Uuid) {
  
}

pub fn edit_user(id:Uuid, username: String, email: String) -> Result<User, String> {
  
  let user = User {
    id,
    email,
    username
  };

  edit_to_file(user.clone())?;
  
  Ok(user)
}

fn edit_to_file(user: User) -> Result<(), String> {
  let users = get_all_users();
  match users.iter().find(|u| u.id == user.id) {
    Some(edit_user) => {
      println!("{:?}", edit_user);
      let new_list: Vec<User> = users
        .iter()
        .map(|u| {
          if u.id == user.id {
              User {
                  id: u.id,
                  email: user.email.clone(),
                  username: user.username.clone(),
              }
          } else {
              u.clone()
          }
        })
        .collect();
      let list_as_json = serde_json::to_string(&new_list).unwrap();
      let mut file = File::create("users.json").expect("Cant create users.json");
      file.write_all(list_as_json.as_bytes()).expect("Cannot write to the file!");
    },
    None => return Err("User not found".to_string()),
  };
  Ok(())
}


fn add_to_file(user: User) -> Result<(), String> {
  let mut users = get_all_users();

  match users.iter().find(|u| u.email == user.email) {
    Some(_) => return Err("User already exists".to_string()),
    None => {
      users.push(user);
      let list_as_json = serde_json::to_string(&users).unwrap();
      let mut file = File::create("users.json").expect("Cant create users.json");
      file.write_all(list_as_json.as_bytes()).expect("Cannot write to the file!");
    },
  };

  Ok(())
}

pub fn get_all_users() -> Vec<User> {
    let file = File::open("users.json").expect("File users.json not found");
    let users: Vec<User> = serde_json::from_reader(file).expect("Fail to read users.json");
    users
}

pub fn select_all() -> Result<Vec<models::User>, String> {
  use crate::schema::users::dsl::*;
  let conn = &mut establish_connection();

  let result = users
    .select(models::User::as_select())
    .load(conn)
    .expect("Error loading users");
  Ok(result)
}

pub fn insert(user: User) -> Result<(), String> {
    use crate::schema::users;

    let uuid = Uuid::new_v4();
  
    let new_user = models::NewUser {
      id: uuid,
      email: user.email,
      username: user.username,
    };

    let conn = &mut establish_connection();

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(models::NewUser::as_returning())
        .get_result(conn)
        .expect("Error saving new post");

    Ok(())
}