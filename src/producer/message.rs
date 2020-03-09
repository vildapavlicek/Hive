
pub mod message {
    use avro_rs::{Codec, Schema, Writer};
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct Message {
        key: u32,
        content: String,
    }

   pub const MESSAGE_AVRO_RAW_SCHEMA: &str = r#"
 {
     "type": "record",
     "name": "message",
     "fields": [
         {"name": "key", "type": "int", "doc": "Key value for kafka"},
         {"name": "content", "type": "string", "doc": "statistics of the Hive of the given day in JSON format"}
     ]
 }
 "#;

    impl Message {
        pub fn new(content: String, key: u32) -> Self {
            Message{
                key: key,
                content: content,
            }
        }

        pub fn get_content(&self) -> &String {
            &self.content
        }

        pub fn get_key(&self) -> u32 {
            self.key
        }

        pub fn to_bytes_as_avro(&self) -> Vec<u8> {
            let message_schema = Schema::parse_str(MESSAGE_AVRO_RAW_SCHEMA).unwrap();
            let mut avro_writer = Writer::with_codec(&message_schema, Vec::new(), Codec::Deflate);
            avro_writer.append_ser(&self).unwrap();
            avro_writer.flush().unwrap();
            avro_writer.into_inner()
        }
    }
}