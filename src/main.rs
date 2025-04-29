use actix_web::{App, HttpServer, middleware};
use dotenv::dotenv;
use api::routes;
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
    })
    .bind((vars.api_host, vars.api_port))?
    .run()
    .await
}
