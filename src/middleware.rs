use std::future::{ready, Ready};

use actix_web::{
    dev::{
        forward_ready,
        Service,
        ServiceRequest,
        ServiceResponse,
        Transform
    },
    http,
    Error,
    HttpResponse,
};
use futures_util::future::LocalBoxFuture;

pub struct CheckAuth;

impl<S, B> Transform<S, ServiceRequest> for CheckAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform: CheckAuthMiddleware<S>;
    type Future: Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckAuthMiddleware { service }))
    }
}

pub struct CheckLoginMiddleware<S> {
    service: S,
}

impl <S, B> Service<ServiceRequest> for CheckAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future: LocalBoxFuture<Output = Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        todo!();
    }
}
