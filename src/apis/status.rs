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
        writeln!(f, "DeviceId: {}", body.device_id)?;
        writeln!(f, "DeviceType: {}", body.device_type)?;
        writeln!(f, "ElectricCurrent: {}", body.electric_current)?;
        writeln!(f, "ElectricityOfDay: {}", body.electricity_of_day)?;
        writeln!(f, "HubDeviceId: {}", body.hub_device_id)?;
        writeln!(f, "Power: {}", body.power)?;
        writeln!(f, "Voltage: {}", body.voltage)?;
        writeln!(f, "Weight: {}", body.weight)
    }
}
#[derive(Deserialize, Debug)]
pub struct Body {
    #[serde(rename = "deviceId")]
    pub device_id: String,

    #[serde(rename = "deviceType")]
    pub device_type: String,

    #[serde(rename = "electricCurrent")]
    pub electric_current: i32,

    #[serde(rename = "electricityOfDay")]
    pub electricity_of_day: i32,

    #[serde(rename = "hubDeviceId")]
    pub hub_device_id: String,

    #[serde(rename = "power")]
    pub power: String,

    #[serde(rename = "voltage")]
    pub voltage: f32,

    #[serde(rename = "weight")]
    pub weight: i32,
}
