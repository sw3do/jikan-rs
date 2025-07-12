pub mod client;
pub mod endpoints;
pub mod error;
pub mod models;
pub mod rate_limiter;

pub use client::JikanClient;
pub use error::{JikanError, Result};

pub mod prelude {
    pub use crate::client::JikanClient;
    pub use crate::error::{JikanError, Result};
    pub use crate::models::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = JikanClient::new();
        assert!(client.check_permit());
    }

    #[tokio::test]
    async fn test_custom_rate_limit() {
        let client = JikanClient::with_rate_limit(1, 1);
        assert!(client.check_permit());
    }
}
