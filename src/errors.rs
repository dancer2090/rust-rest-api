use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Debug)]
pub struct ResponseError {
  pub error: String
}

#[derive(thiserror::Error, Debug)]
#[error("Bad Request: {message}")]
pub struct BadRequest {
    message : String
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Bad request")]
    BadRequest(#[from] BadRequest),
}