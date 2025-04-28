use actix_web::{web, App, HttpServer, middleware};
use dotenv::dotenv;
use envy;

mod routes;
mod environment;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let vars = envy::from_env::<environment::ApiConfig>().unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::default())
            .service(
                web::scope("/ping") 
                    .configure(routes::ping_pong::init_routes)
            )
            .service(
                web::scope("/user") 
                    .configure(routes::user::init_routes)
            )
    })
    .bind((vars.api_host, vars.api_port))?
    .run()
    .await
}
