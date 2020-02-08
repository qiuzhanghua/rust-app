use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpResponse,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use jsonwebtoken::{Validation, DecodingKey};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use crate::auth::AccountService;

pub const MESSAGE_OK: &str = "ok";
pub const MESSAGE_CAN_NOT_FETCH_DATA: &str = "Can not fetch data";
pub const MESSAGE_CAN_NOT_INSERT_DATA: &str = "Can not insert data";
pub const MESSAGE_CAN_NOT_UPDATE_DATA: &str = "Can not update data";
pub const MESSAGE_CAN_NOT_DELETE_DATA: &str = "Can not delete data";
pub const MESSAGE_SIGNUP_SUCCESS: &str = "Signup successfully";
// pub const MESSAGE_SIGNUP_FAILED: &str = "Error while signing up, please try again";
pub const MESSAGE_LOGIN_SUCCESS: &str = "Login successfully";
pub const MESSAGE_LOGIN_FAILED: &str = "Wrong username or password, please try again";
pub const MESSAGE_LOGOUT_SUCCESS: &str = "Logout successfully";
pub const MESSAGE_PROCESS_TOKEN_ERROR: &str = "Error while processing token";
pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";
pub const MESSAGE_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

// Bad request messages
pub const MESSAGE_TOKEN_MISSING: &str = "Token is missing";

// Headers
pub const AUTHORIZATION: &str = "Authorization";

// Misc
pub const EMPTY: &str = "";

// ignore routes
pub const IGNORE_ROUTES: [&str; 3] = ["/api/ping", "/api/auth/signup", "/api/auth/login"];


pub static KEY: [u8; 16] = [23, 54, 78, 30, 91, 56, 02, 43, 63, 31, 87, 48, 55, 11, 99, 130];

#[derive(Serialize, Deserialize)]
pub struct MemToken {
    //   pub iss: String,
    pub exp: i64,
    //    pub sub: String,
    //    pub aud: String,
    //    pub nbf: i64,
    pub iat: i64,
    //    pub jti: String,
    pub  name: String,
    pub login_extra: String, // such as session etc
}

pub struct MemAuthentication;


impl<S, B> Transform<S> for MemAuthentication
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = MemAuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(MemAuthenticationMiddleware { service })
    }
}

pub struct MemAuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for MemAuthenticationMiddleware<S>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let mut authenticate_pass: bool = false;

        // Bypass some account routes
        for ignore_route in IGNORE_ROUTES.iter() {
            if req.path().starts_with(ignore_route) {
                authenticate_pass = true;
            }
        }

        if let Some(authen_header) = req.headers_mut().get(AUTHORIZATION) {
            if let Ok(authen_str) = authen_header.to_str() {
                if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
                    info!("Parsing token...");
                    let token = authen_str[6..authen_str.len()].trim();
                    if let Ok(token_data) = jsonwebtoken::decode::<MemToken>(&token.to_string(),
                                                                             &DecodingKey::from_secret("secret".as_ref()), &Validation::default()) {
                        let x = req.app_data::<Arc<Mutex<AccountService>>>();
                        println!("{:?}", x);  // todo more check
                        if !token_data.claims.name.is_empty() {
                            info!("Valid token");
                            authenticate_pass = true;
                        } else {
                            error!("Invalid token");
                        }
                    }
                }
            }
        }

        if authenticate_pass {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            let data = json!({"message": MESSAGE_INVALID_TOKEN});
            Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized().body(data.to_string()).into_body())
                )
            })
        }
    }
}
