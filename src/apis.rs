pub mod on_off {
    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Response {
        #[serde(rename = "message")]
        pub message: String,

        #[serde(rename = "statusCode")]
        pub status_code: u32,
    }
}

pub mod devices {
    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Response {
        #[serde(rename = "body")]
        pub body: Body,

        #[serde(rename = "message")]
        pub message: String,

        #[serde(rename = "statusCode")]
        pub status_code: u32,
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
}

pub mod status {
    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Response {
        #[serde(rename = "body")]
        pub body: Body,

        #[serde(rename = "message")]
        pub message: String,

        #[serde(rename = "statusCode")]
        pub status_code: u32,
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
}
