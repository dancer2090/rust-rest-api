use actix_web::{web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct Pong {
    result: String
}

async fn ping_pong() -> Result<impl Responder> {
    let obj = Pong {
        result: "pong".to_string(),
    };
    Ok(web::Json(obj))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/ping")
      .route("", web::get().to(ping_pong))
  );
}