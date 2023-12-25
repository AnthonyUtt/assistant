use rumqttc::Event;
use std::error::Error;

mod mqtt_client;
mod messages;
mod actions;
mod handlers;

use messages::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (_mqtt_client, mut event_loop) = mqtt_client::init().await;

    loop {
        let notification = mqtt_client::receive(&mut event_loop).await?;

        // TODO: Match non-publish events to monitor MQTT issues
        match notification {
            Event::Incoming(rumqttc::Packet::Publish(publish)) => {
                let message = Message::new(&publish.topic, publish.payload);
                handlers::handle_message(message);
            }
            _ => {}
        }
    }
}
