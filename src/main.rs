mod hive;
pub mod producer;
use crate::hive::hive::Hive;
pub use producer::producer::MyProducer;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let producer = MyProducer::new("localhost:9092");
    let mut hive = Hive::new(producer);
    hive.run().await;
    Ok(())
}