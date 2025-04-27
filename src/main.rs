use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
mod routes;

#[derive(Deserialize)]
struct InfoQuery {
  username: String
}

#[derive(Deserialize)]
struct InfoParams {
  user_id: u32
}

#[get("/{user_id}")]
async fn hello(path: web::Path<InfoParams>, query: web::Query<InfoQuery>) -> impl Responder {
    let response = format!("{} - {}", path.user_id, query.username);
    HttpResponse::Ok().body(response)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(
                web::scope("/user") 
                    .configure(routes::user::init_routes)
            )
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
