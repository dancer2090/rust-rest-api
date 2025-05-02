use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
  pub id: Uuid,
  pub username: String,
  pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct UserParams {
  pub id: Uuid
}

#[derive(Deserialize, Debug)]
pub struct UserBody {
  pub username: String,
  pub email: String,
}
