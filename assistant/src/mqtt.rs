use rumqttc::{AsyncClient, QoS, MqttOptions, EventLoop};
use std::time::Duration;
use std::env;

pub async fn init() -> (AsyncClient, EventLoop) {
    // Grab namespace for the assistant from ENV (with fallback)
    let topic_namespace = env::var("TOPIC_NAMESPACE").unwrap_or("assistant".to_string());
    // Using # as wildcard to subscribe to all topics under the namespace
    let topic = format!("{}/#", topic_namespace);

    // Create MQTT client named assistant
    let mqtt_host = env::var("MQTT_HOST").unwrap_or("mqtt".to_string());
    let mqtt_port = env::var("MQTT_PORT").unwrap_or("1883".to_string());
    let mut mqtt_options = MqttOptions::new("assistant", mqtt_host, mqtt_port.parse::<u16>().unwrap_or(1883));
    mqtt_options.set_keep_alive(Duration::from_secs(10));

    // Connect to MQTT broker
    let (mqtt_client, event_loop) = AsyncClient::new(mqtt_options, 10);
    // subscribe to all topics under assistant
    mqtt_client.subscribe(topic, QoS::AtLeastOnce).await.expect("Failed to subscribe");

    // Return MQTT client and event loop
    (mqtt_client, event_loop)
}

pub async fn publish(mqtt_client: &AsyncClient, message: MqttMessage) -> Result<(), rumqttc::ClientError> {
    let topic_namespace = env::var("TOPIC_NAMESPACE").unwrap_or("assistant".to_string());
    let topic = format!("{}/{}", topic_namespace, message.topic.as_str());
    let payload = message.payload;

    mqtt_client
        .publish(topic, QoS::AtLeastOnce, false, payload)
        .await
}

pub async fn receive(event_loop: &mut rumqttc::EventLoop) -> Result<rumqttc::Event, rumqttc::ConnectionError> {
    event_loop.poll().await
}

#[derive(Debug, Clone, Copy)]
pub enum MqttTopic {
    // System Messages
    Unknown,
    SystemExit,

    // User Messages
    Prompt,
    Echo,

    // Assistant Messages
    Response,
}

impl MqttTopic {
    pub fn as_str(&self) -> &str {
        match self {
            MqttTopic::Unknown => "unknown",
            MqttTopic::SystemExit => "system/exit",
            MqttTopic::Prompt => "prompt",
            MqttTopic::Echo => "echo",
            MqttTopic::Response => "response",
        }
    }
}

impl From<&str> for MqttTopic {
    fn from(topic: &str) -> Self {
        // Get the topic from the environment if it exists
        let topic_namespace = env::var("TOPIC_NAMESPACE").unwrap_or("assistant".to_string());

        // strip the topic namespace from the topic
        let topic = topic.strip_prefix(&format!("{}/", topic_namespace)).unwrap_or(topic);

        match topic {
            "system/exit" => MqttTopic::SystemExit,
            "prompt" => MqttTopic::Prompt,
            "echo" => MqttTopic::Echo,
            "response" => MqttTopic::Response,
            _ => MqttTopic::Unknown,
        }
    }
}

impl From<String> for MqttTopic {
    fn from(topic: String) -> Self {
        MqttTopic::from(topic.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct MqttMessage {
    pub topic: MqttTopic,
    pub payload: bytes::Bytes,
}
