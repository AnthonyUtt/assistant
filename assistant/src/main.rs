use rumqttc::Event;
use std::error::Error;

mod mqtt_client;
mod messaging;
mod actions;
mod handlers;
mod models;

use messaging::{Message, MessageBus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (_mqtt_client, mut event_loop) = mqtt_client::init().await;
    let bus = messaging::MessageBus::new(None);
    let sender = bus.sender.clone();

    loop {
        let notification = mqtt_client::receive(&mut event_loop).await?;

        // TODO: Match non-publish events to monitor MQTT issues
        match notification {
            Event::Incoming(rumqttc::Packet::Publish(publish)) => {
                let message = Message::new(&publish.topic, publish.payload);
                sender.send(message).expect("Failed to send message");
            }
            _ => {
                println!("Unhandled MQTT event: {:?}", notification);
            }
        }
    }
}
