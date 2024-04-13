pub mod cors {

    use axum::http::{header, HeaderValue, Method};
    use tower_http::cors::CorsLayer;

    pub fn set() -> CorsLayer {
        let ip: String = match dotenv::var("ORIGIN") {
            Ok(ip) => ip,
            Err(_) => "http://localhost:5500".to_string(),
        };

        return CorsLayer::new()
            .allow_origin(ip.parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_headers([header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE]);
    }
}