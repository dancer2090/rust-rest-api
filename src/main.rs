use std::default;

use actix_web::HttpResponse;
use actix_web::{App, HttpServer, middleware, web};
use dotenv::dotenv;
use api::routes;
use api::middlewares;
use api::errors;

mod environment;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let vars = environment::get_api_config();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::default())
            .configure(routes::ping_pong::init_routes)
            .configure(routes::user::init_routes)
            .default_service(
              web::route().to(|| async {
                HttpResponse::NotFound().json(errors::ResponseError { error: "Path not found".to_string() })
              })
            )
    })
    .bind((vars.api_host, vars.api_port))?
    .run()
    .await
}
