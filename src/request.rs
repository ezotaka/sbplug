use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub struct Client {
    api_key: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: String) -> Client {
        Client {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    // TODO: Value must be generics(deserialized data)
    pub async fn post<T>(&self, url: &str, data: Value) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&self.api_key).unwrap());

        let res = self
            .client
            .post(url)
            .headers(headers)
            .json(&data)
            .send()
            .await?;

        let deserialized: T = res.json().await?;
        Ok(deserialized)
    }

    pub async fn get<T>(&self, url: &str) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        // TODO: remove copy and paste
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&self.api_key).unwrap());

        let res = self.client.get(url).headers(headers).send().await?;
        let deserialized: T = res.json().await?;
        Ok(deserialized)
    }
}
