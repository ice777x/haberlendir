use std::env;

use axum::{
    extract::Request,
    http::{self, StatusCode},
    middleware::Next,
    response::Response,
};
pub async fn auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if auth_check(auth_header) {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

fn auth_check(auth_token: &str) -> bool {
    if auth_token
        != env::var("AUTH_TOKEN")
            .expect("Please set AUTH_TOKEN in env file")
            .as_str()
    {
        return false;
    }
    true
    // ...
}
