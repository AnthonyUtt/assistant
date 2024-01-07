use bytes::Bytes;
use std::env;
use flume::{bounded, unbounded, Sender, Receiver};

pub type Callback = Box<dyn FnMut(Message)>;

#[derive(Debug, Clone, Copy)]
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

unsafe impl Send for MessageTopic {}
unsafe impl Sync for MessageTopic {}

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

#[derive(Debug, Clone)]
pub struct Message {
    pub topic: MessageTopic,
    pub payload: Bytes,
}

unsafe impl Send for Message {}
unsafe impl Sync for Message {}

impl Message {
    pub fn new(topic: &str, payload: Bytes) -> Message {
        Message { topic: MessageTopic::from_str(topic), payload }
    }
}

pub struct MessageBus {
    pub sender: Sender<Message>,
    receiver: Receiver<Message>,
    targets: Vec<Callback>,
}

impl MessageBus {
    pub fn new(size: Option<usize>) -> Self {
        let (sender, receiver) = match size {
            Some(size) => bounded(size),
            None => unbounded(),
        };

        MessageBus {
            sender,
            receiver,
            targets: Vec::new(),
        }
    }

    pub fn register(&mut self, callback: impl FnMut(Message) + 'static) {
        self.targets.push(Box::new(callback));
    }

    pub fn process_messages(&mut self, limit: usize) {
        for msg in self.receiver.try_iter().take(limit) {
            self.targets.iter_mut().for_each(|f| f(msg.clone()));
        }
    }
}
