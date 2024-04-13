use axum::Router;

use crate::{
    middlewares::cors,
    router::{router, routes::default},
};

pub async fn app() -> Router {
    let app = Router::new()
        .nest("/", router::set())
        .fallback(default)
        .layer(cors::set());
    return app;
}
