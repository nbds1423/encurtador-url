use super::database::{Database, SClient};

#[allow(dead_code)]
pub async fn execute() {
    let conn = Database::connect().await;

    let result = conn
        .unwrap()
        .batch_execute(
            "
            CREATE TABLE IF NOT EXISTS shortener (
                id SERIAL PRIMARY KEY,
                url VARCHAR(255) UNIQUE,
                short VARCHAR(255),
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            ",
        )
        .await;
    println!("{:?}", result);
}
