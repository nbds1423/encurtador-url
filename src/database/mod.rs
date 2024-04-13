pub mod migrate;
pub mod database {

    use tokio_postgres::{Client, Error, NoTls};

    pub trait SClient {
        async fn connect() -> Result<Client, Error>;
        async fn insert(url: String, short: String) -> Option<bool>;
        async fn get(short: String) -> String;
    }
    pub struct Database;

    impl SClient for Database {
        async fn connect() -> Result<Client, Error> {
            let host = dotenv::var("HOST").unwrap_or("localhost".to_string());
            let user = dotenv::var("USER").unwrap_or("postgres".to_string());
            let url = format!("host={} user={}", host, user);

            let (client, connection) = tokio_postgres::connect(&url, NoTls).await?;

            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                }
            });

            return Ok(client);
        }

        async fn insert(url: String, short: String) -> Option<bool> {
            let conn = Self::connect().await.unwrap();
            let result = conn
                .execute(
                    "INSERT INTO shortener (url, short) VALUES ($1, $2)",
                    &[&url, &short],
                )
                .await;

            return match result {
                Ok(_) => Some(true),
                Err(err) => {
                    if let Some(db_error) = err.as_db_error() {
                        println!("{:?}", db_error.code());
                        if db_error.code().code() == "23505" {
                            return Some(false);
                        }
                    }
                    return None
                }
            };
        }

        async fn get(short: String) -> String {
            let conn = Self::connect().await.unwrap();
            let result = conn
                .query("SELECT * FROM shortener WHERE short = $1", &[&short])
                .await
                .unwrap();

            let mut url = String::new();

            for row in &result {
                url = row.get("url");
            }

            return url;
        }
    }
}
