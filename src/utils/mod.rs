pub mod json;
pub mod config {

    use dotenv;

    pub fn env() -> String {
        let addr: String = dotenv::var("ADDR").unwrap_or(String::from("0.0.0.0"));
        let port: String = dotenv::var("PORT").unwrap_or(String::from("3000"));

        return format!("{}:{}", addr, port);
    }


}

