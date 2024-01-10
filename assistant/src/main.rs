use rumqttc::Event;
use std::error::Error;
use std::sync::mpsc;
use std::thread;
use bytes::Bytes;

mod mqtt;
mod actions;
mod models;

// Overview of program flow:
// Main thread:
// 1. Initialize MQTT client

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (mqtt_client, mut event_loop) = mqtt::init().await;

    // mqtt (main thread) keeps the mqtt_tx and the llm_rx
    // llm (llm thread) keeps the llm_tx and the mqtt_rx
    let (mqtt_tx, mqtt_rx) = mpsc::channel::<String>();
    let (llm_tx, llm_rx) = mpsc::channel::<String>();

    // spawn the llm thread
    thread::spawn(move || {
        use models::InferenceModel;

        // initialize the llm
        let mut llm = InferenceModel::new(llm_tx);

        // loop forever
        loop {
            // wait for a message from mqtt
            let message = mqtt_rx.recv().unwrap();
            println!("LLM received message: {:?}", message);

            // infer a response
            llm.infer(message).unwrap();
        }
    });

    loop {
        use mqtt::{MqttTopic, MqttMessage};
        let notification = mqtt::receive(&mut event_loop).await?;

        // TODO: Match non-publish events to monitor MQTT issues
        match notification {
            Event::Incoming(event) => {
                match event {
                    rumqttc::Packet::Publish(publish) => {
                        let topic = MqttTopic::from(publish.topic);
                        let payload = publish.payload;

                        match topic {
                            MqttTopic::Echo => {
                                println!("Echo: {:?}", payload);
                            }
                            MqttTopic::SystemExit => {
                                println!("System Exit");
                                std::process::exit(0);
                            }
                            MqttTopic::Prompt => {
                                let prompt = String::from_utf8(payload.to_vec()).expect("Failed to convert payload to string");
                                println!("Prompt: {:?}", prompt);
                                mqtt_tx.send(prompt).expect("Failed to send prompt to LLM thread");
                            }
                            _ => {}
                        }
                    }
                    rumqttc::Packet::PingReq => {} // no-op
                    rumqttc::Packet::PingResp => {} // no-op
                    _ => {
                        println!("Unhandled MQTT event: {:?}", event);
                    }
                }
            }
            Event::Outgoing(event) => {
                match event {
                    rumqttc::Outgoing::PingReq => {} // no-op
                    rumqttc::Outgoing::PingResp => {} // no-op
                    _ => {
                        println!("Unhandled MQTT event: {:?}", event);
                    }
                }
            }
        }

        // Check for messages from the llm (non-blocking)
        if let Ok(response) = llm_rx.try_recv() {
            println!("LLM response: {:?}", response);

            // Send the response to the MQTT broker
            let message = MqttMessage { topic: MqttTopic::Response, payload: Bytes::from(response) };
            mqtt::publish(&mqtt_client, message).await?;
        }
    }
}
