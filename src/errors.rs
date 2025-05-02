use serde::Serialize;
use std::fmt::Display;

#[derive(Serialize, Debug)]
pub struct ResponseError {
  pub error: String
}

enum Errors {
  UserExistsError,
}
