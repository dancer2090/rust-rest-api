use std::future::{ready, Ready};
use crate::environment;

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct AuthMiddleware;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareInner<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareInner { service }))
    }
}

pub struct AuthMiddlewareInner<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let auth_header = req.headers().get("Authorization").cloned();

        let fut = self.service.call(req);

        Box::pin(async move {
          match auth_header {
              Some(hdr) if hdr.to_str().unwrap_or("").starts_with("Bearer ") => {
                  let token = hdr.to_str().unwrap().trim_start_matches("Bearer ");
                  let vars = environment::get_api_config();
                  if token == vars.api_token {
                    let res = fut.await?;
                    Ok(res)
                  } else {
                      Err(actix_web::error::ErrorUnauthorized("Invalid token"))
                  }
              }
              _ => Err(actix_web::error::ErrorUnauthorized("Missing token")),
          }
        })
    }
}