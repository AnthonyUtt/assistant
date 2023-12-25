use crate::messages::{Message, MessageTopic};

pub fn handle_message(message: Message) {
    match message.topic {
        MessageTopic::Unknown => {
            println!("Unknown Topic");
        }
        MessageTopic::SystemExit => {
            println!("System Exit");
            std::process::exit(0);
        }
        MessageTopic::Prompt => {
            println!("Prompt");
        }
        MessageTopic::Echo => {
            let payload = String::from_utf8(message.payload.to_vec()).unwrap_or("{}".to_string());
            println!("Echo: {}", payload);
        }
        MessageTopic::Response => {
            println!("Response");
        }
    }
}
