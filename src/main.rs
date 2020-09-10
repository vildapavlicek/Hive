mod hive;

use hive::Hive;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Hive::new().run().await;

    // tokio::signal::ctrl_c().await?;
    // println!("ctrl-c received");

    Ok(())
}
