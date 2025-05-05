use serde::{Deserialize, Serialize};
use uuid::{Uuid, Version};
use validator::{Validate, ValidationError};

fn validate_uuid_v4(uuid: &Uuid) -> Result<(), ValidationError> {
  if uuid.get_version() == Some(Version::Random) {
      Ok(())
  } else {
      let mut err = ValidationError::new("invalid_uuid_v4");
      err.message = Some("Must be a valid UUID v4".into());
      Err(err)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
  pub id: Uuid,
  pub username: String,
  pub email: String,
}

#[derive(Deserialize, Debug, Validate)]
pub struct UserParams {
  #[validate(custom(function = "validate_uuid_v4"))]
  pub id: Uuid
}

#[derive(Deserialize, Debug, Validate)]
pub struct CreateUserBody {
  #[validate(length(min = 1, max = 100))]
  pub username: String,
  #[validate(email)]
  pub email: String,
}


#[derive(Deserialize, Debug, Validate)]
pub struct UpdateUserBody {
  #[validate(length(min = 1, max = 100))]
  pub username: String,
  #[validate(email)]
  pub email: String,
}