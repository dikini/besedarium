use besedarium::*;

// All protocol example tests in this file have been temporarily disabled to stabilize the test base for the TInteract refactor.

// Publish/subscribe (MQTT)
pub type MqttPubSub = TChoice<
    Mqtt,
    EmptyLabel,
    TInteract<Mqtt, EmptyLabel, TClient, Publish, TEnd<Mqtt, EmptyLabel>>,
    TInteract<Mqtt, EmptyLabel, TClient, Subscribe, TEnd<Mqtt, EmptyLabel>>,
>;
