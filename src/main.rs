use sbplug::{commands, request};
use std::env;
use structopt::StructOpt;

/// CLI tool to control SwitchBot Plug
#[derive(Debug, StructOpt)]
#[structopt(name = "sbplug")]
enum Cli {
    /// Turn the device on
    #[structopt(name = "on")]
    On(commands::On),

    /// Turn the device off
    #[structopt(name = "off")]
    Off(commands::Off),

    /// Get status of the device
    #[structopt(name = "status")]
    Status(commands::Status),

    /// Get list of the devices
    #[structopt(name = "list")]
    List(commands::List),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let api_key = match env::var("SWITCHBOT_API_KEY") {
        Ok(key) => key,
        Err(_) => panic!("SWITCHBOT_API_KEY not found."),
    };

    let client = request::Client::new(api_key);

    let result = match args {
        Cli::On(cmd) => cmd.exec(&client).await?,
        Cli::Off(cmd) => cmd.exec(&client).await?,
        Cli::Status(cmd) => cmd.exec(&client).await?,
        Cli::List(cmd) => cmd.exec(&client).await?,
    };

    println!("{}", result);

    Ok(())
}
