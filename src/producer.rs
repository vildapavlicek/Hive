
pub mod producer{
    use crate::hive::statistics::stats::{Statistics, DeathType};
    use futures::*;
    use std::sync::mpsc::Receiver;
    use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
    use rdkafka::producer::{FutureProducer, FutureRecord};
    use rdkafka::message::{OwnedHeaders, Headers};
    use rdkafka::consumer::{ConsumerContext, Rebalance, StreamConsumer, Consumer, CommitMode};
    use rdkafka::error::KafkaResult;

    pub struct Message {
        key: u32,
        content: String,
        stored: bool,
    }

    impl Message {
        pub fn new(content: String, key: u32) -> Self {
            Message{
                key: key,
                content: content,
                stored: false,
            }
        }

        fn get_content(&self) -> &String {
            &self.content
        }
    }

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

            for message in self.rx.recv() {
                produce(&self.producer, topic_name, message).await
            }
        }
    }

    pub async fn produce(producer: &FutureProducer, topic_name: &str, message: Message) {
        let future = producer.send(
            FutureRecord::to(topic_name)
            .payload(message.get_content())
            .key(&format!("Key {}", message.key))
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