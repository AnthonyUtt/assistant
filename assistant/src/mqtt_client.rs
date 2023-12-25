use rumqttc::{AsyncClient, QoS, MqttOptions, EventLoop};
use std::time::Duration;
use std::env;

pub async fn init() -> (AsyncClient, EventLoop) {
    // Grab namespace for the assistant from ENV (with fallback)
    let topic_namespace = env::var("TOPIC_NAMESPACE").unwrap_or("assistant".to_string());
    // Using # as wildcard to subscribe to all topics under the namespace
    let topic = format!("{}/#", topic_namespace);

    // Create MQTT client named assistant
    let mut mqtt_options = MqttOptions::new("assistant", "mqtt", 1883);
    mqtt_options.set_keep_alive(Duration::from_secs(10));

    // Connect to MQTT broker
    let (mqtt_client, event_loop) = AsyncClient::new(mqtt_options, 10);
    // subscribe to all topics under assistant
    mqtt_client.subscribe(topic, QoS::AtLeastOnce).await.expect("Failed to subscribe");

    // Return MQTT client and event loop
    (mqtt_client, event_loop)
}

pub async fn publish(mqtt_client: &AsyncClient, topic: &str, payload: &str) -> Result<(), rumqttc::ClientError> {
    mqtt_client
        .publish(topic, QoS::AtLeastOnce, false, payload)
        .await
}

pub async fn receive(event_loop: &mut rumqttc::EventLoop) -> Result<rumqttc::Event, rumqttc::ConnectionError> {
    event_loop.poll().await
}
