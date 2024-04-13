use app::app;
use dotenv as env;

#[allow(unused_imports)]
use database::migrate;

mod app;
mod controller;
mod database;
mod middlewares;
mod modal;
mod router;
mod server;
mod utils;

#[tokio::main]
async fn main() {

    //migrate::execute().await;

    env::dotenv().ok();
    server::start().await;
}
