
pub mod producer{
    use crate::hive::statistics::stats::{Statistics, DeathType};
    use futures::*;
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
        fn new(content: String, key: u32) -> Self {
            Message{
                key: key,
                content: content,
                stored: false,
            }
        }

        fn get_content(&self) -> &String {
            &self.content
        }

        fn set_stored(&mut self) {
            self.stored = true;
        }

        fn is_stored(&self) -> bool {
            self.stored
        }
    }

    pub struct MyProducer {
        producer: FutureProducer,
        data_container: Vec<Message>
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
                data_container: vec![],
            }
        }

        pub fn get_producer(&mut self) -> &FutureProducer {
            &self.producer
        }
/*
        pub async fn produce(&mut self, topic_name: &str, message: &mut Message) {
            let future = self.producer.send(
                FutureRecord::to(topic_name)
                .payload(message.get_content())
                .key(&format!("Key {}", message.key))
                .headers(OwnedHeaders::new().add("header_key", "header_value")),
                 0)
                 .map(move |delivery_status| {
                    // This will be executed onw the result is received
                    //println!("Delivery status for message received");
                    delivery_status
                });
                
           //let _ =  future.await.expect("failed to produce message to kafka");
           match future.await {
               Ok(_) => message.set_stored(),
               Err(e) => println!("Encountered error storing message {}", e)
           }

        }
*/
        pub fn add_message(&mut self, content: String, id: u32) {
            self.data_container.push(
                Message::new(content, id)
            );
        }

        pub async fn publish(&mut self, topic_name: &str) {   
            for message in self.data_container.iter_mut() {
                produce(&self.producer, topic_name, message).await;
            }

            self.data_container.retain(|m| !m.is_stored());
        }
    }

    pub async fn produce(producer: &FutureProducer, topic_name: &str, message: &mut Message) {
        let future = producer.send(
            FutureRecord::to(topic_name)
            .payload(message.get_content())
            .key(&format!("Key {}", message.key))
            .headers(OwnedHeaders::new().add("header_key", "header_value")),
             0)
             .map(move |delivery_status| {
                // This will be executed onw the result is received
                //println!("Delivery status for message received");
                delivery_status
            });
            
       //let _ =  future.await.expect("failed to produce message to kafka");
       match future.await {
           Ok(_) => message.set_stored(),
           Err(e) => println!("Encountered error storing message {}", e)
       }

    }

}