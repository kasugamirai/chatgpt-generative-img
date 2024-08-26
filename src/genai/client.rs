use super::ImageGenerationRequest;
use super::ImageGenerationResponse;
use super::ModelConfiguration;
use super::Result;

use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use reqwest::Proxy;

pub struct openai_client {
    client: Client,
    config: ModelConfiguration,
}

impl openai_client {
    pub fn new<S: Into<String>>(api_key: S) -> Result<Self> {
        Self::new_with_config(api_key, ModelConfiguration::default())
    }

    pub fn new_with_proxy<S: Into<String>>(api_key: S, proxy: Proxy) -> Result<Self> {
        Self::new_with_config_proxy(api_key, ModelConfiguration::default(), proxy)
    }

    pub fn new_with_config<S: Into<String>>(
        api_key: S,
        config: ModelConfiguration,
    ) -> Result<openai_client> {
        let api_key = api_key.into();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_bytes(format!("Bearer {api_key}").as_bytes())?,
        );
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .timeout(config.timeout)
            .build()?;
        Ok(Self { client, config })
    }

    pub fn new_with_config_proxy<S: Into<String>>(
        api_key: S,
        config: ModelConfiguration,
        proxy: Proxy,
    ) -> Result<openai_client> {
        let api_key = api_key.into();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_bytes(format!("Bearer {api_key}").as_bytes())?,
        );
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .timeout(config.timeout)
            .proxy(proxy)
            .build()?;
        Ok(Self { client, config })
    }

    pub async fn send_img_generation_request<S: Into<String>>(
        &self,
        message: S,
    ) -> Result<Vec<ImageGenerationResponse>> {
        let response = self
            .client
            .post(self.config.api_url.clone())
            .header("Content-Type", "application/json")
            .json(&ImageGenerationRequest {
                prompt: message.into(),
                n: self.config.count,
                size: self.config.size.clone(),
            })
            .send()
            .await?;
        Ok(response.json().await?)
    }
}
