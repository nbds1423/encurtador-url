use axum::{
    body::Body,
    extract::Request,
    http::{HeaderValue, Response, StatusCode},
    middleware::Next,
    Json,
};
use serde_json::Value;

use crate::utils::json;

pub async fn authenticate(
    req: Request,
    next: Next,
) -> Result<Response<Body>, (StatusCode, Json<Value>)> {
    return match validate(req.headers().get("authorization")) {
        Some(_) => {
            let response = next.run(req).await;
            Ok(response)
        }
        None => Err(json::error(StatusCode::UNAUTHORIZED, "unauthorized")),
    };
}

fn validate(token: Option<&HeaderValue>) -> Option<()> {
    match token {
        Some(token) => {
            let env: String = dotenv::var("TOKEN").ok()?;
            let token: String = token.to_str().ok()?.to_string();

            if token.is_empty() || token != env {
                return None;
            }

            return Some(());
        }
        _ => None,
    }
}
