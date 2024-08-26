use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Request failed with status code: {0}")]
    RequestFailed(reqwest::StatusCode),

    #[error("Failed to parse the response body: {0}")]
    ResponseParsingFailed(String),

    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Header value error: {0}")]
    HeaderValue(#[from] reqwest::header::InvalidHeaderValue),
}
