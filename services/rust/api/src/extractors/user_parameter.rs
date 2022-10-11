use log::{
    info,
    debug,
    error
};

use futures::{
    future::{
        ok,
        err,
        Ready
    }
};

use actix_web::{
    dev::Payload,
    http::StatusCode, 
    HttpMessage,
    HttpRequest,
    FromRequest,
    ResponseError
};

use common::user::User;


#[derive(Debug)]
pub enum UserError {
    InternalServerError
}

impl std::fmt::Display for UserError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            UserError::InternalServerError => write!(f, "internal server error")
        }
    }
}


#[derive(Debug, Clone)]
pub struct UserParameter {
    user: User
}

impl UserParameter {

    pub fn new(user: User) -> Self {
        return Self {
            user: user
        };
    }

    pub fn user(&self) -> User {
        return self.user.clone();
    }
}

impl ResponseError for UserError {

    fn status_code(&self) -> StatusCode {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
}

impl FromRequest for UserParameter {
    type Error = UserError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if request.extensions().contains::<UserParameter>() {
            if let Some(user) = request.extensions().get::<UserParameter>() {
                return ok(user.clone());
            }
        }

        return err(UserError::InternalServerError);
    }
}

