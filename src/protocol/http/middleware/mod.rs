use std::sync::Arc;

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse},
    Error, HttpRequest, HttpResponse,
};
use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use futures_util::future::LocalBoxFuture;
use tracing::info;

// use crate::common::errors::Res;

#[derive(Debug)]
pub struct FormatLoggering<Req, Res> {
    pub xtime: chrono::NaiveDateTime,
    pub path: String,
    pub headers: String,
    pub xclient: String,
    pub req: Req,
    pub resp: Res,
}

pub struct Loggering<S> {
    pub service: S,
}

// pub struct

impl<S, B> Service<ServiceRequest> for Loggering<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<self::ServiceResponse<B>, self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // let arcreq = Arc::new(req);
        let fut = self.service.call(req);
        info!("hello from req");
        Box::pin(async move {
            let res = fut.await?;
            info!("hello from response");
            Ok(res)
        })
    }
}
