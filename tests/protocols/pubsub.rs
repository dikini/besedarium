use besedarium::*;

// Publish/subscribe (MQTT)
pub type MqttPubSub = TChoice<
    Mqtt,
    EmptyLabel,
    TInteract<Mqtt, EmptyLabel, TClient, Publish, TEnd<Mqtt, EmptyLabel>>,
    TInteract<Mqtt, EmptyLabel, TClient, Subscribe, TEnd<Mqtt, EmptyLabel>>,
>;
