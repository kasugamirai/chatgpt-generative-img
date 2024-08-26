use chatgpt_generative_img::prelude::*;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber
    tracing_subscriber::fmt::init();

    // Attempt to create an OpenaiClient
    let client = match OpenaiClient::new("OPENAI_API_KEY") {
        Ok(client) => client,
        Err(err) => {
            error!("Failed to create OpenaiClient: {:?}", err);
            return;
        }
    };

    // Define the prompt for image generation
    let prompt = "A beautiful sunset over the city";

    // Send image generation request and handle errors
    let images = match client.send_img_generation_request(prompt).await {
        Ok(response) => response,
        Err(err) => {
            error!("Failed to generate images: {:?}", err);
            return;
        }
    };

    // Log the resulting image URLs
    for (i, image) in images.data.iter().enumerate() {
        info!("Image {}: {}", i + 1, image.url);
    }
}
