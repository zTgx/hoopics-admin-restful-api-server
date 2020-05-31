use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse};
use futures::future::{ok, Future, Ready};

use crate::models::response::ResponseBody;
use crate::config::whitelist::WHITELIST;
use crate::models::user_token::UserToken;

pub struct Authentication;

impl<S: 'static, B> Transform<S> for Authentication
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct AuthenticationMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>
        + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest; 
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let path = req.path();
        //check if path exists in whitelist
        match WHITELIST.get_key_value(path) {
            Some(result) => {
                if *(result.1) {
                    //需要校验api-key
                    let token = req.headers().get("api-key");
                    if token.is_none() || UserToken::decode_token(&token.unwrap().to_str().unwrap().to_string()).is_err() {
                        return Box::pin(async move {
                            Ok(req.into_response(
                                HttpResponse::Unauthorized()
                                    .json(ResponseBody::new(401, "未授权的接口访问", ""))
                                    .into_body(),
                            ))
                        });
                    }            
                }
            },
            None => {
                // info!("不需要校验api-key");
            }
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
