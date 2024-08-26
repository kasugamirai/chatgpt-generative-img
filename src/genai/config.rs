use std::time::Duration;
use url::Url;

pub struct ModelConfiguration {
    pub count: u8,
    pub api_url: Url,
    pub size: String,
    pub timeout: Duration,
}

impl Default for ModelConfiguration {
    fn default() -> Self {
        let api_url = Url::parse("https://api.openai.com/v1/images/generations")
            .expect("Failed to parse default API URL");

        Self {
            count: 1,
            api_url,
            size: "1024x1024".to_string(),
            timeout: Duration::from_secs(30),
        }
    }
}

impl ModelConfiguration {
    pub fn new(count: u8, api_url: url::Url, size: String, timeout: std::time::Duration) -> Self {
        Self {
            count,
            api_url,
            size,
            timeout,
        }
    }
}
