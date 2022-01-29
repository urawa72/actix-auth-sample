use actix_session::{Session, UserSession};
use actix_web::HttpResponse;
use futures::future::{ok, Future, Ready};
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;

#[derive(Clone, Debug)]
pub struct AuthService;

impl<S, B> Transform<S> for AuthService
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &mut self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut srv = self.service.clone();
        Box::pin(async move {
            let path = req.path().to_string();
            if path.find("/login").is_none()
                && get_user_id_from_session(&req.get_session()).is_none()
            {
                // 余計なkeyがredisに残らないようにする
                req.get_session().renew();
                return Ok(req.into_response(HttpResponse::Unauthorized().finish().into_body()));
            }

            let res = srv.call(req);
            res.await
        })
    }
}

fn get_user_id_from_session(session: &Session) -> Option<String> {
    match session.get::<String>("user_id") {
        Ok(user_id) => user_id,
        _ => return None,
    }
}
