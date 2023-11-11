use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_identity::IdentityExt;
use actix_session::{Session, SessionExt};
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;

use crate::{app_error::AppError, db::models::user::AuthenticatedUser};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.

/// A middleware to handle Authentication and Authorization
#[derive(Clone)]
pub struct RequireAuthentication {
    pub admin_only: bool,
}
impl Default for RequireAuthentication {
    /// # Default settings
    /// - Allows Authenticated users to this route else 401
    /// - Allows all types of users
    /// - If you want role based usage use set_functions
    ///
    /// ## Available Set Functions
    /// ```rust
    /// RequireAuthentication.set_admin_only()
    /// ```
    fn default() -> Self {
        Self { admin_only: false }
    }
}

impl RequireAuthentication {
    /// Allow only admin users to access the route
    pub fn set_admin_only(mut self, enabled: bool) -> Self {
        self.admin_only = enabled;
        self
    }
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S: 'static, B> Transform<S, ServiceRequest> for RequireAuthentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequireAuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequireAuthenticationMiddleware {
            service: Rc::new(service),
            admin_only: self.admin_only,
        }))
    }
}

pub struct RequireAuthenticationMiddleware<S> {
    service: Rc<S>,
    admin_only: bool,
}

impl<S, B> Service<ServiceRequest> for RequireAuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = Rc::clone(&self.service);
        let admin_only = self.admin_only;

        Box::pin(async move {
            // Check if the user is logged in with the identity middleware
            check_user_identity(&req)?;

            // User in the session with {id, username, is_admin}
            // You may also modify the Authenticated user and add roles to it than just Admin
            let auth_user = get_session_user(req.get_session())?;

            // check if the route is admin_only and if the user is an admin
            if admin_only && !auth_user.is_admin {
                return Err(AppError::Forbidden("You are not an admin").into());
            }

            // Add the session user into the request extensions so the next routes can access it
            req.extensions_mut().insert(auth_user);
            let res = svc.call(req).await?; // process the request and return the response(async)
            Ok(res)
        })
    }
}

fn check_user_identity(req: &ServiceRequest) -> Result<(), AppError> {
    let identity = req.get_identity();
    match identity {
        Ok(uid) => match uid.id() {
            Ok(_) => Ok(()),
            Err(_) => Err(AppError::Unauthorized),
        },
        Err(_) => Err(AppError::Unauthorized),
    }
}

fn get_session_user(session: Session) -> Result<AuthenticatedUser, AppError> {
    let session_user = session.get::<AuthenticatedUser>(&crate::APP_CONFIG.auth_cookie_key);
    match session_user {
        Ok(auth_user) => match auth_user {
            Some(user) => Ok(user),
            None => Err(AppError::Unauthorized),
        },
        Err(_) => Err(AppError::Unauthorized),
    }
}
