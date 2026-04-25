use thiserror::Error;

/// Convenience alias used throughout the crate
pub type Res<T> = Result<T, Error>;

/// All errors that can be returned by the Yandex Smart Home client
#[derive(Error, Debug)]
pub enum Error {
    /// Failed to parse a URL
    #[error("Error while parsing url.")]
    UrlParse(#[from] url::ParseError),
    /// HTTP transport error from `reqwest`
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// Failed to deserialize the API response
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    /// The OAuth token could not be used as an HTTP header value
    #[error("Invalid authentication token.")]
    AuthToken,
}
