use std::fmt::Display;

use serde_derive::Deserialize;

use super::ApiResponse;

#[derive(Deserialize, Debug)]
pub struct Response {
    #[serde(rename = "body")]
    pub body: Body,

    #[serde(rename = "message")]
    pub message: String,

    #[serde(rename = "statusCode")]
    pub status_code: u32,
}

impl ApiResponse for Response {}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body = &self.body;
        write!(
            f,
            "DeviceId: {}, DeviceName: {}, DeviceType: {}, EnableCloudService: {}, HubDeviceId: {}",
            body.device_list.as_ref().unwrap()[0].device_id,
            body.device_list.as_ref().unwrap()[0].device_name,
            body.device_list.as_ref().unwrap()[0].device_type,
            body.device_list.as_ref().unwrap()[0].enable_cloud_service,
            body.device_list.as_ref().unwrap()[0].hub_device_id,
        )
    }
}

#[derive(Deserialize, Debug)]
pub struct Body {
    #[serde(default, rename = "deviceList")]
    pub device_list: Option<Vec<Device>>,

    #[serde(default, rename = "infraredRemoteList")]
    pub infrared_remote_list: Option<Vec<serde_json::Value>>,
}

#[derive(Deserialize, Debug)]
pub struct Device {
    #[serde(rename = "deviceId")]
    pub device_id: String,

    #[serde(rename = "deviceName")]
    pub device_name: String,

    #[serde(rename = "deviceType")]
    pub device_type: String,

    #[serde(rename = "enableCloudService")]
    pub enable_cloud_service: bool,

    #[serde(rename = "hubDeviceId")]
    pub hub_device_id: String,
}
