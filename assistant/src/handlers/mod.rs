use crate::messages::{Message, MessageTopic};
use crate::State;
use crate::models::infer_async;

pub fn handle_message(state: &mut State, message: Message) {
    match message.topic {
        MessageTopic::Unknown => {
            println!("Unknown Topic");
        }
        MessageTopic::SystemExit => {
            println!("System Exit");
            std::process::exit(0);
        }
        MessageTopic::Prompt => {
            let prompt = String::from_utf8(message.payload.to_vec());
            match prompt {
                Ok(prompt) => {
                    println!("Prompt: {}", prompt);
                    infer_async(prompt);
                }
                Err(e) => {
                    println!("Prompt Error: {}", e);
                }
            }
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
