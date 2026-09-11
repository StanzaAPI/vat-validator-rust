//! Official Rust Client for EU & UK VAT Validator & Rate Engine
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, ACCEPT};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct VatValidatorClient {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
    pub tool_url: &'static str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub code: Option<String>,
    pub tool_url: Option<String>,
    pub upgrade_url: Option<String>,
}

impl VatValidatorClient {
    pub fn new(api_key: Option<String>, base_url: Option<String>) -> Self {
        let key = api_key
            .or_else(|| env::var("STANZA_API_KEY").ok())
            .or_else(|| env::var("API_KEY").ok())
            .unwrap_or_default();
        let base = base_url.unwrap_or_else(|| "https://api.stanzaapi.com/vat-validator".to_string());

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .unwrap_or_default();

        Self {
            client,
            api_key: key,
            base_url: base.trim_end_matches('/').to_string(),
            tool_url: "https://stanzaapi.com/tools/vat-validator",
        }
    }

    async fn send_request<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        body: Option<serde_json::Value>,
    ) -> Result<ApiResponse<T>, reqwest::Error> {
        let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        if !self.api_key.is_empty() {
            if let Ok(val) = HeaderValue::from_str(&self.api_key) {
                headers.insert("x-api-key", val);
            }
        }

        let mut req = self.client.request(method, &url).headers(headers);
        if let Some(json) = body {
            req = req.json(&json);
        }

        let resp = req.send().await?;
        let mut result = resp.json::<ApiResponse<T>>().await?;
        if result.tool_url.is_none() {
            result.tool_url = Some(self.tool_url.to_string());
        }
        if result.upgrade_url.is_none() {
            result.upgrade_url = Some(self.tool_url.to_string());
        }
        Ok(result)
    }

    pub async fn get_health<T: for<'de> Deserialize<'de>>(&self) -> Result<ApiResponse<T>, reqwest::Error> {
        self.send_request("/health", reqwest::Method::GET, None).await
    }

    pub async fn validate<T: for<'de> Deserialize<'de>>(&self, payload: serde_json::Value) -> Result<ApiResponse<T>, reqwest::Error> {
        self.send_request("/api/v1/validate", reqwest::Method::POST, Some(payload)).await
    }

    pub async fn parse<T: for<'de> Deserialize<'de>>(&self, payload: serde_json::Value) -> Result<ApiResponse<T>, reqwest::Error> {
        self.send_request("/api/v1/validate", reqwest::Method::POST, Some(payload)).await
    }
}
