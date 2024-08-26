mod client;
mod config;
mod error;
mod types;

pub use client::OpenaiClient;
pub use config::ModelConfiguration;
pub use error::Error;
pub use types::ImageGenerationRequest;
pub use types::ImageGenerationResponse;

pub type Result<T> = std::result::Result<T, Error>;
