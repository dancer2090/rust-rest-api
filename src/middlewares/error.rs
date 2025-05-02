use actix_web::{
  dev::ServiceRequest, dev::ServiceResponse, error, middleware, web, App, HttpResponse,
  HttpServer, Result,
};
use actix_web::http::StatusCode;

pub fn render_500<B>(res: ServiceResponse<B>) -> actix_web::Result<middleware::ErrorHandlerResponse<B>> {
  let response = HttpResponse::InternalServerError()
      .body("Internal Server Error");
  
  Ok(middleware::ErrorHandlerResponse::Response(
      res.into_response(response.map_into_right_body()),
  ))
}


pub fn render_404<B>(res: ServiceResponse<B>) -> actix_web::Result<middleware::ErrorHandlerResponse<B>> {
  let response = HttpResponse::InternalServerError()
      .body("Not Found");
  
  Ok(middleware::ErrorHandlerResponse::Response(
      res.into_response(response.map_into_right_body()),
  ))
}