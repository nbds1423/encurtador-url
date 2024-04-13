pub mod response {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Response {
        pub data: Message,
    }
    
    #[derive(Serialize, Deserialize)]
    pub struct Error {
        pub error: Message,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Message {
        pub code: u16,
        pub message: String,
    }
}
