mod client;
mod config;
mod error;
mod types;

use reqwest::Client;
use std::env;

pub use client::openai_client;
pub use config::ModelConfiguration;
pub use error::Error;
pub use types::ImageGenerationRequest;
pub use types::ImageGenerationResponse;

pub type Result<T> = std::result::Result<T, Error>;

/// Generate images using the OpenAI API.

pub async fn generate_images(
    client: &Client,
    prompt: &str,
    n: u8,
    size: &str,
) -> Result<Vec<String>> {
    let api_key = env::var("OPENAI_API_KEY")
        .map_err(|_| Error::MissingEnvVar("OPENAI_API_KEY".to_string()))?;

    let request_body = ImageGenerationRequest {
        prompt: prompt.to_string(),
        n,
        size: size.to_string(),
    };

    let response = client
        .post("https://api.openai.com/v1/images/generations")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;

    if response.status().is_success() {
        let response_body: ImageGenerationResponse = response
            .json()
            .await
            .map_err(|err| Error::ResponseParsingFailed(err.to_string()))?;
        Ok(response_body
            .data
            .into_iter()
            .map(|image| image.url)
            .collect())
    } else {
        Err(Error::RequestFailed(response.status()))
    }
}
