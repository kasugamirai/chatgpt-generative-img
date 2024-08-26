use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ImageData {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct ImageGenerationResponse {
    pub created: u64,
    pub data: Vec<ImageData>,
}

#[derive(Serialize)]
pub struct ImageGenerationRequest {
    pub prompt: String,
    pub n: u8,
    pub size: String,
}
