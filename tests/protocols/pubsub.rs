use playground::*;

// Publish/subscribe (MQTT)
pub type MqttPubSub = TChoice<Mqtt,
    TInteract<Mqtt, TClient, Publish, TEnd<Mqtt>>,
    TInteract<Mqtt, TClient, Subscribe, TEnd<Mqtt>>
>;
