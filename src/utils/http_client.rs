use crate::utils::Result;
use reqwest::Client;
use std::time::Duration;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct HttpClient {
    client: Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Trading-Signals-Backend/1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: base_url.to_string(),
        }
    }

    pub async fn get(&self, endpoint: &str, params: Option<Vec<(&str, &str)>>) -> Result<Value> {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let mut request = self.client.get(&url);
        
        if let Some(params_list) = params {
            for (key, value) in params_list {
                request = request.query(&[(key, value)]);
            }
        }

        let response = request.send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("HTTP {}: {}", status, text).into());
        }

        let json: Value = response.json().await?;
        Ok(json)
    }

    pub async fn get_with_headers(
        &self,
        endpoint: &str,
        headers: Vec<(&str, &str)>,
    ) -> Result<Value> {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let mut request = self.client.get(&url);
        
        for (key, value) in headers {
            request = request.header(key, value);
        }

        let response = request.send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("HTTP {}: {}", status, text).into());
        }

        let json: Value = response.json().await?;
        Ok(json)
    }

    pub async fn post(
        &self,
        endpoint: &str,
        body: &Value,
        headers: Option<Vec<(&str, &str)>>,
    ) -> Result<Value> {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let mut request = self.client.post(&url).json(body);
        
        if let Some(headers_list) = headers {
            for (key, value) in headers_list {
                request = request.header(key, value);
            }
        }

        let response = request.send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("HTTP {}: {}", status, text).into());
        }

        let json: Value = response.json().await?;
        Ok(json)
    }
}