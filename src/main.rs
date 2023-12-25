use rumqttc::{Client, MqttOptions, QoS};
use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut mqtt_options = MqttOptions::new("assistant", "localhost", 1883);
    mqtt_options.set_keep_alive(Duration::from_secs(10));

    let (mut mqtt_client, _event_loop) = Client::new(mqtt_options, 10);
    mqtt_client.publish("assistant/prompt", QoS::AtLeastOnce, false, "Hello, world!".to_string())?;

    Ok(())
}
