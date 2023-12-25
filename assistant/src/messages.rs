use bytes::Bytes;
use std::env;

pub enum MessageTopic {
    // System Messages
    Unknown,
    SystemExit,

    // User Messages
    Prompt,
    Echo,

    // Assistant Messages
    Response,
}

impl MessageTopic {
    pub fn from_str(topic: &str) -> MessageTopic {
        // Get the topic from the environment if it exists
        let topic_namespace = env::var("TOPIC_NAMESPACE").unwrap_or("assistant".to_string());

        // strip the topic namespace from the topic
        let topic = topic.strip_prefix(&format!("{}/", topic_namespace)).unwrap_or(topic);

        match topic {
            "system/exit" => MessageTopic::SystemExit,
            "prompt" => MessageTopic::Prompt,
            "echo" => MessageTopic::Echo,
            "response" => MessageTopic::Response,
            _ => MessageTopic::Unknown,
        }
    }
}

pub struct Message {
    pub topic: MessageTopic,
    pub payload: Bytes,
}

impl Message {
    pub fn new(topic: &str, payload: Bytes) -> Message {
        Message { topic: MessageTopic::from_str(topic), payload }
    }
}
