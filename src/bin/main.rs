use chatgpt_generative_img::prelude::*;
use reqwest::Client;
use tracing::error;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let client = Client::new();

    match generate_images(&client, "A cute anime girl and a pikachu", 1, "1024x1024").await {
        Ok(urls) => {
            for url in urls {
                info!("Image URL: {}", url);
            }
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}
