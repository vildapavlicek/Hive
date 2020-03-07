mod hive;
pub mod producer;
use crate::hive::hive::Hive;
pub use producer::producer::{MyProducer, Message};
use std::sync::mpsc::{self, Sender, Receiver, channel};
use std::thread;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let rt = tokio::runtime::Runtime::new().unwrap();
    let (tx, rx): (Sender<Message>, Receiver<Message>) = channel();

    println!("init hive_task");
    let hive_task = async {
        Hive::new(tx).run().await;
    };
    rt.spawn(hive_task);

    let mut producer = MyProducer::new("localhost:9092", rx);
    producer.publish("hive").await;
    

    tokio::signal::ctrl_c().await?;
    println!("ctrl-c received");

    Ok(())
}