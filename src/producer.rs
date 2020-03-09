
mod message;

pub mod producer{
    use futures::*;
    use rdkafka::config::{ClientConfig};
    use rdkafka::producer::{FutureProducer, FutureRecord};
    use rdkafka::message::{OwnedHeaders};
    use tokio::sync::mpsc::{Receiver};
    pub use super::message::message::Message;

    pub struct MyProducer {
        producer: FutureProducer,
        rx: Receiver<Message>,
    }

    impl MyProducer {
        pub fn new(brokers: &str, rx: Receiver<Message>) -> Self {
            let producer: FutureProducer = ClientConfig::new()
                .set("bootstrap.servers", brokers)
                .set("message.timeout.ms", "5000")
                .create()
                .expect("Producer creation error");
    
    
            MyProducer {
                producer: producer,
                rx: rx,
            }
        }

        pub fn get_producer(&mut self) -> &FutureProducer {
            &self.producer
        }

        pub async fn publish(&mut self, topic_name: &str) {
            loop {
                for message in self.rx.recv().await {
                    let future = self.producer.send(
                        FutureRecord::to(topic_name)
                        .payload(&message.to_bytes_as_avro())
                        .key(&format!("Key {}", &message.get_key()))
                        .headers(OwnedHeaders::new().add("header_key", "header_value")),
                         0)
                         .map(move |delivery_status| {
                            delivery_status
                        });
                        
                    match future.await {
                        Ok(_) => (),
                        Err(e) => println!("Error publishing message: {}", e),
                    }
                }
            }
        }
    }
}