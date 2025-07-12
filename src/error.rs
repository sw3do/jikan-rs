use thiserror::Error;

pub type Result<T> = std::result::Result<T, JikanError>;

#[derive(Error, Debug)]
pub enum JikanError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("API returned error: {status} - {message}")]
    ApiError { status: u16, message: String },

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Resource not found")]
    NotFound,

    #[error("URL parsing failed: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
