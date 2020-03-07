mod hive;
pub mod producer;
use crate::hive::hive::Hive;
pub use producer::producer::{MyProducer, Message};
use tokio::sync::mpsc::{self, Sender, Receiver};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let rt = tokio::runtime::Runtime::new().unwrap();
    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel(100);
    let mut producer = MyProducer::new("localhost:9092", rx);
    let mut hive = Hive::new(tx);

    println!("init hive_task");
    rt.spawn(
        async move {
            hive.run().await;
        }
    );

    rt.spawn(
        async move {
            producer.publish("hive").await;
        }
    );

    tokio::signal::ctrl_c().await?;
    println!("ctrl-c received");

    Ok(())
}