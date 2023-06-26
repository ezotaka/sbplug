use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::env;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "sbplug")]
enum Cli {
    #[structopt(name = "on")]
    On { device_id: String },
    #[structopt(name = "off")]
    Off { device_id: String },
    #[structopt(name = "status")]
    Status { device_id: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    // Retrieve the API key from the environment variable
    let api_key = match env::var("SWITCHBOT_API_KEY") {
        Ok(key) => key,
        Err(_) => panic!("SWITCHBOT_API_KEY not found."),
    };

    let client = reqwest::Client::new();

    // Construct the headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&api_key)?);

    match args {
        Cli::On { device_id } => {
            // Construct the URL
            let url = format!(
                "https://api.switch-bot.com/v1.0/devices/{}/commands",
                device_id
            );

            // Construct the JSON data
            let data = json!({
                "command": "turnOn",
                "parameter": "default",
                "commandType": "command"
            });

            // Send the POST request
            let res = client
                .post(&url)
                .headers(headers.clone())
                .json(&data)
                .send()
                .await?;

            println!("Response: {}", res.status());

            // Get the response text and parse it as JSON
            let body: serde_json::Value = res.json().await?;
            // Print the JSON in a pretty format
            println!("Body: {}", serde_json::to_string_pretty(&body)?);
        }
        Cli::Off { device_id } => {
            // Construct the URL
            let url = format!(
                "https://api.switch-bot.com/v1.0/devices/{}/commands",
                device_id
            );

            // Construct the JSON data
            let data = json!({
                "command": "turnOff",
                "parameter": "default",
                "commandType": "command"
            });

            // Send the POST request
            let res = client
                .post(&url)
                .headers(headers.clone())
                .json(&data)
                .send()
                .await?;

            println!("Response: {}", res.status());

            // Get the response text and parse it as JSON
            let body: serde_json::Value = res.json().await?;
            // Print the JSON in a pretty format
            println!("Body: {}", serde_json::to_string_pretty(&body)?);
        }
        Cli::Status { device_id } => {
            // Construct the URL
            let url = format!(
                "https://api.switch-bot.com/v1.0/devices/{}/status",
                device_id
            );

            // Send the GET request
            let res = client.get(&url).headers(headers).send().await?;

            println!("Response: {}", res.status());

            // Get the response text and parse it as JSON
            let body: serde_json::Value = res.json().await?;
            // Print the JSON in a pretty format
            println!("Body: {}", serde_json::to_string_pretty(&body)?);
        }
    }

    Ok(())
}
