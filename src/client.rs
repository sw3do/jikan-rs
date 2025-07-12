use crate::error::{JikanError, Result};
use crate::rate_limiter::JikanRateLimiter;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;

pub struct JikanClient {
    client: Client,
    base_url: String,
    rate_limiter: JikanRateLimiter,
}

impl Default for JikanClient {
    fn default() -> Self {
        Self::new()
    }
}

impl JikanClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.jikan.moe/v4".to_string(),
            rate_limiter: JikanRateLimiter::new(),
        }
    }

    pub fn with_rate_limit(requests_per_second: u32, burst_size: u32) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.jikan.moe/v4".to_string(),
            rate_limiter: JikanRateLimiter::with_custom_rate(requests_per_second, burst_size),
        }
    }

    pub fn with_base_url(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            rate_limiter: JikanRateLimiter::new(),
        }
    }

    async fn make_request<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.rate_limiter.wait_for_permit().await;

        let url = format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            endpoint.trim_start_matches('/')
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "jikan-rs/0.1.0")
            .send()
            .await
            .map_err(JikanError::Http)?;

        self.handle_response(response).await
    }

    async fn handle_response<T>(&self, response: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let status = response.status();

        if status.is_success() {
            let text = response.text().await.map_err(JikanError::Http)?;
            serde_json::from_str(&text).map_err(JikanError::Json)
        } else {
            match status.as_u16() {
                429 => Err(JikanError::RateLimitExceeded),
                404 => Err(JikanError::NotFound),
                400 => Err(JikanError::ApiError {
                    status: status.as_u16(),
                    message: "Bad Request".to_string(),
                }),
                500..=599 => Err(JikanError::ApiError {
                    status: status.as_u16(),
                    message: "Server Error".to_string(),
                }),
                _ => Err(JikanError::Unknown(format!("HTTP {}", status.as_u16()))),
            }
        }
    }

    pub async fn get<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.make_request(endpoint).await
    }

    pub fn check_permit(&self) -> bool {
        self.rate_limiter.check_permit()
    }
}
