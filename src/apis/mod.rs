pub mod commands;
pub mod devices;
pub mod status;

use std::{env, fmt::Display};

use anyhow::Context;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

pub struct Client {
    client: reqwest::Client,
    headers: HeaderMap,
}

impl Client {
    pub fn new() -> Client {
        let api_key = match env::var("SWITCHBOT_API_KEY") {
            Ok(key) => key,
            Err(_) => panic!("SWITCHBOT_API_KEY not found."),
        };

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&api_key).unwrap());

        Client {
            client: reqwest::Client::new(),
            headers,
        }
    }

    async fn execute_request<T>(
        &self,
        request: impl Fn(&reqwest::Client) -> reqwest::RequestBuilder,
    ) -> anyhow::Result<T>
    where
        T: DeserializeOwned + ApiResponse,
    {
        let res = request(&self.client)
            .headers(self.headers.clone())
            .send()
            .await
            .with_context(|| "API error")?;

        let deserialized = res.json::<T>().await.with_context(|| "JSON parse error")?;
        Ok(deserialized)
    }

    // TODO: Value must be generics(deserialized data)
    pub async fn post<T>(&self, url: &str, data: Value) -> anyhow::Result<T>
    where
        T: DeserializeOwned + ApiResponse,
    {
        self.execute_request(move |client| client.post(url).json(&data))
            .await
    }

    pub async fn get<T>(&self, url: &str) -> anyhow::Result<T>
    where
        T: DeserializeOwned + ApiResponse,
    {
        self.execute_request(move |client| client.get(url)).await
    }

    pub async fn get_devices(&self) -> anyhow::Result<devices::Response> {
        let url = "https://api.switch-bot.com/v1.0/devices";
        self.get(url).await
    }

    pub async fn get_status(&self, device_id: &str) -> anyhow::Result<status::Response> {
        let url = format!(
            "https://api.switch-bot.com/v1.0/devices/{}/status",
            device_id
        );
        self.get(&url).await
    }

    pub async fn post_on(&self, device_id: &str) -> anyhow::Result<commands::Response> {
        self.post_command(device_id, "turnOn", "default").await
    }

    pub async fn post_off(&self, device_id: &str) -> anyhow::Result<commands::Response> {
        self.post_command(device_id, "turnOff", "default").await
    }

    async fn post_command(
        &self,
        device_id: &str,
        command: &'static str,
        parameter: &str,
    ) -> anyhow::Result<commands::Response> {
        let url = format!(
            "https://api.switch-bot.com/v1.0/devices/{}/commands",
            device_id
        );

        let data = json!({
            "command": command,
            "parameter": parameter,
            "commandType": "command"
        });

        self.post(&url, data).await
    }
}

pub trait ApiResponse: Display {}
