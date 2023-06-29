use reqwest::Error;
use serde::de::DeserializeOwned;
use serde_json::json;
use structopt::StructOpt;

use crate::{
    apis::{devices, on_off, status},
    request::Client,
};

async fn exec_on_off<T: DeserializeOwned>(
    client: &Client,
    device_id: &str,
    command: &'static str,
) -> Result<T, Error> {
    let url = format!(
        "https://api.switch-bot.com/v1.0/devices/{}/commands",
        device_id
    );

    let data = json!({
        "command": command,
        "parameter": "default",
        "commandType": "command"
    });

    Ok(client.post(&url, data).await?)
}

#[derive(Debug, StructOpt)]
pub struct On {
    pub device_id: String,
}

impl On {
    pub async fn exec(&self, client: &Client) -> Result<String, Box<dyn std::error::Error>> {
        let res: on_off::Response = exec_on_off(client, &self.device_id, "turnOn").await?;
        Ok(res.message)
    }
}
#[derive(Debug, StructOpt)]
pub struct Off {
    pub device_id: String,
}

impl Off {
    pub async fn exec(&self, client: &Client) -> Result<String, Box<dyn std::error::Error>> {
        let res: on_off::Response = exec_on_off(client, &self.device_id, "turnOff").await?;
        Ok(res.message)
    }
}

#[derive(Debug, StructOpt)]
pub struct Status {
    pub device_id: String,
}

impl Status {
    pub async fn exec(&self, client: &Client) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.switch-bot.com/v1.0/devices/{}/status",
            self.device_id
        );
        let res: status::Response = client.get(&url).await?;
        Ok(format!("{}", res.body.power))
    }
}

#[derive(Debug, StructOpt)]
pub struct List {}

impl List {
    pub async fn exec(&self, client: &Client) -> Result<String, Box<dyn std::error::Error>> {
        let url = "https://api.switch-bot.com/v1.0/devices";
        let res: devices::Response = client.get(url).await?;
        let mut devices = Vec::new();
        if let Some(device_list) = res.body.device_list {
            for (i, device) in device_list.into_iter().enumerate() {
                devices.push(format!(
                    "{})\t{}\t{}",
                    i + 1,
                    device.device_id,
                    device.device_name
                ))
            }
        }
        Ok(devices.join("\n"))
    }
}
