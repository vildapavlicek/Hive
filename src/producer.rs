



pub mod producer{
    use crate::hive::statistics::stats::{Statistics, DeathType};
    use futures::*;
    use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
    use rdkafka::producer::{FutureProducer, FutureRecord};
    use rdkafka::message::{OwnedHeaders, Headers};
    use rdkafka::{ClientContext, TopicPartitionList, Message};
    use rdkafka::consumer::{ConsumerContext, Rebalance, StreamConsumer, Consumer, CommitMode};
    use rdkafka::error::KafkaResult;

    pub struct MyProducer {
        producer: FutureProducer
    }

    impl MyProducer {
        pub fn new(brokers: &str) -> Self {
            let producer: FutureProducer = ClientConfig::new()
                .set("bootstrap.servers", brokers)
                .set("message.timeout.ms", "5000")
                .create()
                .expect("Producer creation error");
    
    
            MyProducer {
                producer: producer,
            }
        }

        pub async fn produce(&self, topic_name: &str, stats: &Statistics) {
            let future = self.producer.send(
                FutureRecord::to(topic_name)
                .payload(&stats.report())
                .key(&format!("Key {}", stats.get_day()))
                .headers(OwnedHeaders::new().add("header_key", "header_value")),
                 0);/*
                 .map(move |delivery_status| {
                    // This will be executed onw the result is received
                    println!("Delivery status for message {} received", stats.get_day());
                    delivery_status
                });
                */
                
           let _ =  future.await.expect("failed to produce message to kafka");
        }
    }
}