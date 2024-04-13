pub mod routes;
pub mod router {
    use axum::{
        middleware,
        routing::{get, post},
        Router,
    };

    use super::routes::{create, redirect};
    use crate::controller::auth;

    pub fn set() -> Router {
        return Router::new()
            .route("/create", post(create))
            .route_layer(middleware::from_fn(auth::authenticate))
            .route("/:path", get(redirect))
    }
}
