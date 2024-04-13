use crate::{app, utils};

pub async fn start() {
    let app = app().await;

    let listener = tokio::net::TcpListener::bind(utils::config::env())
        .await
        .unwrap();

    println!("Server is Online!");
    axum::serve(listener, app).await.unwrap();
}
