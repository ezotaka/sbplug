use sbplug::cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let result = cli::run().await?;
    println!("{}", result);
    Ok(())
}
