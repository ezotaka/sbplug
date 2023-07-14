use crate::apis;
use std::fmt::{Display, Write};
use std::str::FromStr;
use structopt::StructOpt;

/// CLI tool to control SwitchBot Plug
#[derive(Debug, StructOpt)]
#[structopt(name = "sbplug")]
pub enum Command {
    /// Turn the device on
    #[structopt(name = "on")]
    On { id: String },

    /// Turn the device off
    #[structopt(name = "off")]
    Off { id: String },

    /// Get status of the device
    #[structopt(name = "status")]
    Status { id: String },

    /// Get list of the devices
    #[structopt(name = "list")]
    List,
}

pub async fn run() -> anyhow::Result<String> {
    Ok(match Command::from_args() {
        Command::On { id } => run_on_command(&id).await?,
        Command::Off { id } => run_off_command(&id).await?,
        Command::Status { id } => run_status_command(&id).await?,
        Command::List => run_list_command().await?,
    })
}

struct Device {
    number: usize,
    id: String,
    name: String,
    power: String,
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{})\t{}\t{}\t{}",
            self.number, self.id, self.name, self.power
        )
    }
}

async fn run_list_command() -> anyhow::Result<String> {
    let mut result = String::new();
    for device in get_devices().await? {
        writeln!(result, "{}", device)?;
    }
    Ok(result)
}

async fn get_device_id(id: String) -> String {
    match usize::from_str(&id) {
        Ok(number) if number < 1000 => match get_devices().await {
            Ok(devices) => {
                if devices.len() >= number {
                    devices[number - 1].id.clone()
                } else {
                    id
                }
            }
            Err(_) => id,
        },
        _ => id,
    }
}

async fn get_devices() -> anyhow::Result<Vec<Device>> {
    let client = apis::Client::new();
    let devices_res = client.get_devices().await?;

    let mut result = vec![];
    if let Some(devices) = devices_res.body.device_list {
        let plug_devices = devices.iter().filter(|d| d.device_type.contains("Plug"));
        for (i, device) in plug_devices.enumerate() {
            // TODO: make concurrently
            let status_res = client.get_status(&device.device_id).await?;
            result.push(Device {
                number: i + 1,
                id: device.device_id.clone(),
                name: device.device_name.clone(),
                power: status_res.body.power,
            })
        }
    }
    Ok(result)
}

async fn run_on_command(id: &str) -> anyhow::Result<String> {
    let client = apis::Client::new();
    let device_id = get_device_id(id.to_string()).await;
    let res = client.post_on(&device_id).await?;
    Ok(res.message)
}

async fn run_off_command(id: &str) -> anyhow::Result<String> {
    let client = apis::Client::new();
    let device_id = get_device_id(id.to_string()).await;
    let res = client.post_off(&device_id).await?;
    Ok(res.message)
}

async fn run_status_command(id: &str) -> anyhow::Result<String> {
    let client = apis::Client::new();
    let device_id = get_device_id(id.to_string()).await;
    let res = client.get_status(&device_id).await?;
    Ok(format!("{}", res))
}
