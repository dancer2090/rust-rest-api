pub mod routes;
pub mod scopes;
pub mod middlewares;
pub mod environment;
pub mod errors;

pub mod models;
pub mod schema;

use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
  let vars = environment::get_api_config();
  
  let database_url = vars.database_url;
  PgConnection::establish(&database_url)
      .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}