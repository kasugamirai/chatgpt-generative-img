mod error;
pub use error::ImageGenerationError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
pub struct ImageGenerationRequest {
    pub prompt: String,
    pub n: u8,
    pub size: String,
}

#[derive(Deserialize)]
pub struct ImageData {
    pub url: String,
}

#[derive(Deserialize)]
pub struct ImageGenerationResponse {
    pub created: u64,
    pub data: Vec<ImageData>,
}

pub async fn generate_images(
    client: &Client,
    prompt: &str,
    n: u8,
    size: &str,
) -> Result<Vec<String>, ImageGenerationError> {
    let api_key = env::var("OPENAI_API_KEY")
        .map_err(|_| ImageGenerationError::MissingEnvVar("OPENAI_API_KEY".to_string()))?;

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
            .map_err(|err| ImageGenerationError::ResponseParsingFailed(err.to_string()))?;
        Ok(response_body
            .data
            .into_iter()
            .map(|image| image.url)
            .collect())
    } else {
        Err(ImageGenerationError::RequestFailed(response.status()))
    }
}
