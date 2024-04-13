use axum::{extract::Request, http::StatusCode, response::Redirect, Json};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    database::database::{Database, SClient},
    utils::json::{error, response},
};

#[derive(Debug, Deserialize)]
pub struct Link {
    url: String,
    short: String,
}

pub async fn default() -> (StatusCode, Json<Value>) {
    return error(StatusCode::NOT_FOUND, "Hello, World!");
}

pub async fn redirect(req: Request) -> Redirect {
    let path = req.uri().to_string().replace("/", "");
    let result = Database::get(path).await;

    if result.is_empty() {
        return Redirect::permanent("/");
    }

    return Redirect::permanent(result.as_str());
}

pub async fn create(Json(payload): Json<Link>) -> (StatusCode, Json<Value>) {
    let Link { url, short } = payload;
    let url = url
        .trim()
        .replace("https://", "")
        .replace("http://", "")
        .to_string();
    let short = short.trim().to_lowercase().to_string();

    match Database::insert(url, short).await {
        Some(true) => response(StatusCode::OK, "ok"),
        Some(false) => error(StatusCode::CONFLICT, "path.duplicate"),
        None => error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "interal.server.error",
        ),
    }
}
