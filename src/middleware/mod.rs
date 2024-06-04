#![cfg(feature="ssr")]

use actix_web::body::{MessageBody, EitherBody, BoxBody};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::http::header::{self, HeaderValue};
use actix_web::{HttpResponse, HttpRequest};
use actix_web::web::Data;
use actix_web_lab::middleware::Next;
use sqlx::{Pool, Row, Sqlite};
use crate::server::PROTECTED_ROUTES;

pub(crate) async fn check_auth_for_protected_routes<B: MessageBody + 'static>(
    req: ServiceRequest,
    next: Next<B>
) -> Result<ServiceResponse<EitherBody<B, BoxBody>>, Error> {
    let pool = req.app_data::<Data<Pool<Sqlite>>>().unwrap().clone();

    if !PROTECTED_ROUTES.contains(req.path()) {
        return next.call(req).await.map(ServiceResponse::map_into_left_body);
    }

    let user_from_cookie = req.cookie("user");
    let token_from_cookie = req.cookie("token");

    if user_from_cookie.is_none() || token_from_cookie.is_none() {
        return Ok(redirect_to_login(req.request().clone()));
    }

    let q = sqlx::query("\
        WITH subq AS (SELECT user_id FROM session WHERE token = ?)\
        SELECT name FROM (user JOIN subq ON user.id = subq.user_id)\
    ")
    .bind(token_from_cookie.unwrap().value())
    .fetch_one(pool.as_ref())
    .await.map_err(|e| {
        ErrorInternalServerError(e.to_string())
    })?;

    let user_from_token: &str = q.get("name");

    if user_from_token != user_from_cookie.unwrap().value() {
        return Ok(redirect_to_login(req.request().clone()));        
    }

    return next.call(req).await.map(ServiceResponse::map_into_left_body);
}

fn redirect_to_login<B: MessageBody + 'static>(req: HttpRequest) -> ServiceResponse<EitherBody<B, BoxBody>> {
    let name = header::LOCATION;
    let value = HeaderValue::from_str("/login").unwrap();
    let res = HttpResponse::Found().insert_header((name, value)).finish();
    ServiceResponse::new(req, res).map_into_right_body()
}
