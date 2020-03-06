mod hive;
pub mod producer;
use crate::hive::hive::Hive;
pub use producer::producer::MyProducer;

fn main() {
    println!("Hello, world!");
    let producer = MyProducer::new("host.docker.internal:9092");
    let mut hive = Hive::new(producer);
    hive.run();
}