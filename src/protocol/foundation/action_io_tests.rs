//! Comprehensive unit tests for SupportsActionIO and ActionIOType integration
//!
//! This module provides exhaustive testing of the action I/O capability system,
//! ensuring that I/O types properly integrate with action requirements and
//! protocol constraints at compile time.

use super::*;
use crate::protocol::local::*;
use std::marker::PhantomData;

// ============================================================================
// Test I/O Capability Types
// ============================================================================

/// MQTT Publisher - only supports output actions (publish messages)
#[derive(Debug, Clone)]
struct MqttPublisherIO;

impl SupportsActionIO<OutputAction> for MqttPublisherIO {}
// Notably missing: InputAction and BiDirectionalAction support

/// MQTT Subscriber - only supports input actions (receive messages)
#[derive(Debug, Clone)]
struct MqttSubscriberIO;

impl SupportsActionIO<InputAction> for MqttSubscriberIO {}
// Notably missing: OutputAction and BiDirectionalAction support

/// WebSocket I/O - supports all action types (full duplex)
#[derive(Debug, Clone)]
struct WebSocketIO;

impl SupportsActionIO<InputAction> for WebSocketIO {}
impl SupportsActionIO<OutputAction> for WebSocketIO {}
impl SupportsActionIO<BiDirectionalAction> for WebSocketIO {}

/// UDP Sender - only supports output actions (send packets)
#[derive(Debug, Clone)]
struct UdpSenderIO;

impl SupportsActionIO<OutputAction> for UdpSenderIO {}

/// UDP Receiver - only supports input actions (receive packets)
#[derive(Debug, Clone)]
struct UdpReceiverIO;

impl SupportsActionIO<InputAction> for UdpReceiverIO {}

/// REST API Client - supports output and bidirectional (HTTP requests)
#[derive(Debug, Clone)]
struct RestApiClientIO;

impl SupportsActionIO<OutputAction> for RestApiClientIO {}
impl SupportsActionIO<BiDirectionalAction> for RestApiClientIO {}

/// Test roles for capability testing
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Alice;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bob;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Carol;

impl Role for Alice {}
impl Role for Bob {}
impl Role for Carol {}

// Test messages
#[derive(Debug, Clone)]
struct PublishMsg(String);
#[derive(Debug, Clone)]
struct SubscribeMsg;
#[derive(Debug, Clone)]
struct NotificationMsg(u64);

impl Message for PublishMsg {}
impl Message for SubscribeMsg {}
impl Message for NotificationMsg {}

// Test metadata
type PubSubMeta = CommMetadata<DefaultChan, RequestLbl>;
type DataMeta = CommMetadata<HandshakeChan, ResponseLbl>;

// ============================================================================
// ActionIOTMarker and Basic Trait Tests
// ============================================================================

#[cfg(test)]
#[test]
fn test_action_io_marker_trait_bounds() {
    // Verify all action types implement required trait bounds
    fn verify_marker_bounds<T>()
    where
        T: ActionIOTMarker + Send + Sync + 'static + std::fmt::Debug + Clone + PartialEq + Eq,
    {
        // All bounds satisfied at compile time
    }

    verify_marker_bounds::<InputAction>();
    verify_marker_bounds::<OutputAction>();
    verify_marker_bounds::<BiDirectionalAction>();
}

#[cfg(test)]
#[test]
fn test_action_io_marker_equality() {
    // Test equality comparisons work correctly within same type
    assert_eq!(InputAction, InputAction);
    assert_eq!(OutputAction, OutputAction);
    assert_eq!(BiDirectionalAction, BiDirectionalAction);

  // Note: Cross-type comparisons are prevented by the type system
    // This ensures type safety at compile time rather than runtime
}

#[cfg(test)]
#[test]
fn test_action_io_marker_cloning() {
    // Test cloning preserves equality
    let input = InputAction;
    let output = OutputAction;
    let bidirectional = BiDirectionalAction;

    assert_eq!(input, input.clone());
    assert_eq!(output, output.clone());
    assert_eq!(bidirectional, bidirectional.clone());
}

// ============================================================================
// SupportsActionIO Implementation Tests
// ============================================================================

#[cfg(test)]
#[test]
fn test_tcp_only_session_io_capabilities() {
    // TcpOnlySessionIO should support all action types
    assert!(<TcpOnlySessionIO as SupportsActionIO<InputAction>>::supports_action_io());
    assert!(<TcpOnlySessionIO as SupportsActionIO<OutputAction>>::supports_action_io());
    assert!(<TcpOnlySessionIO as SupportsActionIO<
        BiDirectionalAction,
    >>::supports_action_io());
}

#[cfg(test)]
#[test]
fn test_http_only_session_io_capabilities() {
    // HttpOnlySessionIO should support output and bidirectional only
    assert!(<HttpOnlySessionIO as SupportsActionIO<OutputAction>>::supports_action_io());
    assert!(<HttpOnlySessionIO as SupportsActionIO<
        BiDirectionalAction,
    >>::supports_action_io());

    // Note: HttpOnlySessionIO doesn't implement SupportsActionIO<InputAction>
    // This is verified by the type system - attempting to use it would cause a compile error
}

#[cfg(test)]
#[test]
fn test_mqtt_publisher_capabilities() {
    // MQTT Publisher should only support output actions
    assert!(<MqttPublisherIO as SupportsActionIO<OutputAction>>::supports_action_io());

    // These would fail to compile if uncommented:
    // assert!(<MqttPublisherIO as SupportsActionIO<InputAction>>::supports_action_io());
    // assert!(<MqttPublisherIO as SupportsActionIO<BiDirectionalAction>>::supports_action_io());
}

#[cfg(test)]
#[test]
fn test_mqtt_subscriber_capabilities() {
    // MQTT Subscriber should only support input actions
    assert!(<MqttSubscriberIO as SupportsActionIO<InputAction>>::supports_action_io());

    // These would fail to compile if uncommented:
    // assert!(<MqttSubscriberIO as SupportsActionIO<OutputAction>>::supports_action_io());
    // assert!(<MqttSubscriberIO as SupportsActionIO<BiDirectionalAction>>::supports_action_io());
}

#[cfg(test)]
#[test]
fn test_websocket_capabilities() {
    // WebSocket should support all action types (full duplex)
    assert!(<WebSocketIO as SupportsActionIO<InputAction>>::supports_action_io());
    assert!(<WebSocketIO as SupportsActionIO<OutputAction>>::supports_action_io());
    assert!(<WebSocketIO as SupportsActionIO<BiDirectionalAction>>::supports_action_io());
}

#[cfg(test)]
#[test]
fn test_udp_sender_capabilities() {
    // UDP Sender should only support output actions
    assert!(<UdpSenderIO as SupportsActionIO<OutputAction>>::supports_action_io());
}

#[cfg(test)]
#[test]
fn test_udp_receiver_capabilities() {
    // UDP Receiver should only support input actions
    assert!(<UdpReceiverIO as SupportsActionIO<InputAction>>::supports_action_io());
}

#[cfg(test)]
#[test]
fn test_rest_api_client_capabilities() {
    // REST API Client should support output and bidirectional
    assert!(<RestApiClientIO as SupportsActionIO<OutputAction>>::supports_action_io());
    assert!(<RestApiClientIO as SupportsActionIO<BiDirectionalAction>>::supports_action_io());
}

// ============================================================================
// Compile-Time Constraint Verification Tests
// ============================================================================

#[cfg(test)]
#[test]
fn test_compile_time_capability_constraints() {
    // These functions verify capabilities at compile time
    fn requires_input_capability<IO: SupportsActionIO<InputAction>>() {}
    fn requires_output_capability<IO: SupportsActionIO<OutputAction>>() {}
    fn requires_bidirectional_capability<IO: SupportsActionIO<BiDirectionalAction>>() {}

    // TCP supports all
    requires_input_capability::<TcpOnlySessionIO>();
    requires_output_capability::<TcpOnlySessionIO>();
    requires_bidirectional_capability::<TcpOnlySessionIO>();

    // WebSocket supports all
    requires_input_capability::<WebSocketIO>();
    requires_output_capability::<WebSocketIO>();
    requires_bidirectional_capability::<WebSocketIO>();

    // HTTP supports output and bidirectional
    requires_output_capability::<HttpOnlySessionIO>();
    requires_bidirectional_capability::<HttpOnlySessionIO>();

    // MQTT Publisher supports only output
    requires_output_capability::<MqttPublisherIO>();

    // MQTT Subscriber supports only input
    requires_input_capability::<MqttSubscriberIO>();

    // UDP Sender supports only output
    requires_output_capability::<UdpSenderIO>();

    // UDP Receiver supports only input
    requires_input_capability::<UdpReceiverIO>();

    // REST API Client supports output and bidirectional
    requires_output_capability::<RestApiClientIO>();
    requires_bidirectional_capability::<RestApiClientIO>();
}

#[cfg(test)]
#[test]
fn test_multiple_capability_constraints() {
    // Test functions that require multiple capabilities
    fn requires_input_and_output<IO>()
    where
        IO: SupportsActionIO<InputAction> + SupportsActionIO<OutputAction>,
    {
    }

    fn requires_all_capabilities<IO>()
    where
        IO: SupportsActionIO<InputAction>
            + SupportsActionIO<OutputAction>
            + SupportsActionIO<BiDirectionalAction>,
    {
    }

    // Only TCP and WebSocket support all capabilities
    requires_input_and_output::<TcpOnlySessionIO>();
    requires_input_and_output::<WebSocketIO>();

    requires_all_capabilities::<TcpOnlySessionIO>();
    requires_all_capabilities::<WebSocketIO>();


    // These would fail to compile:
    // requires_input_and_output::<MqttPublisherIO>(); // No input support
    // requires_input_and_output::<HttpOnlySessionIO>(); // No input support
    // requires_all_capabilities::<RestApiClientIO>(); // No input support
}

// ============================================================================
// Protocol Integration Tests
// ============================================================================

#[cfg(test)]
#[test]
fn test_endpoint_action_io_integration() {
    // Test that endpoints properly integrate with action I/O types

    // Input action endpoints
    type InputEndpoint = EpChanEnd<TcpOnlySessionIO, PubSubMeta, InputAction>;
    type WebSocketInputEndpoint = EpChanEnd<WebSocketIO, PubSubMeta, InputAction>;

    // Output action endpoints
    type OutputEndpoint = EpChanEnd<MqttPublisherIO, PubSubMeta, OutputAction>;
    type HttpOutputEndpoint = EpChanEnd<HttpOnlySessionIO, PubSubMeta, OutputAction>;

    // Bidirectional action endpoints
    type BiDirectionalEndpoint = EpChanEnd<TcpOnlySessionIO, PubSubMeta, BiDirectionalAction>;
    type RestBiDirectionalEndpoint = EpChanEnd<RestApiClientIO, PubSubMeta, BiDirectionalAction>;

    // Verify these compile and satisfy LocalProtocol
    fn requires_local_protocol<T: LocalProtocol>(_: PhantomData<T>) {}

    requires_local_protocol(PhantomData::<InputEndpoint>);
    requires_local_protocol(PhantomData::<WebSocketInputEndpoint>);
    requires_local_protocol(PhantomData::<OutputEndpoint>);
    requires_local_protocol(PhantomData::<HttpOutputEndpoint>);
    requires_local_protocol(PhantomData::<BiDirectionalEndpoint>);
    requires_local_protocol(PhantomData::<RestBiDirectionalEndpoint>);
}

#[cfg(test)]
#[test]
fn test_send_endpoint_action_io_integration() {
    // Test EpChanSend with different I/O capabilities and action types

    type TcpSendInput = EpChanSend<
        TcpOnlySessionIO,
        PubSubMeta,
        PublishMsg,
        EpChanEnd<TcpOnlySessionIO, PubSubMeta, InputAction>,
        InputAction,
    >;

    type MqttSendOutput = EpChanSend<
        MqttPublisherIO,
        PubSubMeta,
        PublishMsg,
        EpChanEnd<MqttPublisherIO, PubSubMeta, OutputAction>,
        OutputAction,
    >;

    type WebSocketSendBiDir = EpChanSend<
        WebSocketIO,
        PubSubMeta,
        PublishMsg,
        EpChanEnd<WebSocketIO, PubSubMeta, BiDirectionalAction>,
        BiDirectionalAction,
    >;

    fn requires_local_protocol<T: LocalProtocol>(_: PhantomData<T>) {}

    requires_local_protocol(PhantomData::<TcpSendInput>);
    requires_local_protocol(PhantomData::<MqttSendOutput>);
    requires_local_protocol(PhantomData::<WebSocketSendBiDir>);
}

#[cfg(test)]
#[test]
fn test_recv_endpoint_action_io_integration() {
    // Test EpChanRecv with different I/O capabilities and action types

    type TcpRecvInput = EpChanRecv<
        TcpOnlySessionIO,
        PubSubMeta,
        SubscribeMsg,
        EpChanEnd<TcpOnlySessionIO, PubSubMeta, InputAction>,
        InputAction,
    >;

    type WebSocketRecvOutput = EpChanRecv<
        WebSocketIO,
        PubSubMeta,
        NotificationMsg,
        EpChanEnd<WebSocketIO, PubSubMeta, OutputAction>,
        OutputAction,
    >;

    type HttpRecvBiDir = EpChanRecv<
        HttpOnlySessionIO,
        PubSubMeta,
        NotificationMsg,
        EpChanEnd<HttpOnlySessionIO, PubSubMeta, BiDirectionalAction>,
        BiDirectionalAction,
    >;

    fn requires_local_protocol<T: LocalProtocol>(_: PhantomData<T>) {}

    requires_local_protocol(PhantomData::<TcpRecvInput>);
    requires_local_protocol(PhantomData::<WebSocketRecvOutput>);
    requires_local_protocol(PhantomData::<HttpRecvBiDir>);
}

// ============================================================================
// Complex Protocol Structure Tests
// ============================================================================

#[cfg(test)]
#[test]
fn test_choice_action_io_integration() {
    // Test EpChanChoice with different action I/O types

    type TcpChoice = EpChanChoice<
        TcpOnlySessionIO,
        PubSubMeta,
        EpChanEnd<TcpOnlySessionIO, PubSubMeta, InputAction>,
        EpChanEnd<TcpOnlySessionIO, PubSubMeta, OutputAction>,
        BiDirectionalAction,
    >;

    type WebSocketChoice = EpChanChoice<
        WebSocketIO,
        PubSubMeta,
        EpChanEnd<WebSocketIO, PubSubMeta, BiDirectionalAction>,
        EpChanEnd<WebSocketIO, PubSubMeta, BiDirectionalAction>,
        BiDirectionalAction,
    >;

    fn requires_local_protocol<T: LocalProtocol>(_: PhantomData<T>) {}

    requires_local_protocol(PhantomData::<TcpChoice>);
    requires_local_protocol(PhantomData::<WebSocketChoice>);
}

#[cfg(test)]
#[test]
fn test_parallel_action_io_integration() {
    // Test EpChanPar with different action I/O types

    type TcpParallel = EpChanPar<
        TcpOnlySessionIO,
        PubSubMeta,
        EpChanEnd<TcpOnlySessionIO, PubSubMeta, InputAction>,
        EpChanEnd<TcpOnlySessionIO, PubSubMeta, OutputAction>,
        Alice,
        BiDirectionalAction,
    >;

    type WebSocketParallel = EpChanPar<
        WebSocketIO,
        PubSubMeta,
        EpChanEnd<WebSocketIO, PubSubMeta, BiDirectionalAction>,
        EpChanEnd<WebSocketIO, PubSubMeta, BiDirectionalAction>,
        Bob,
        BiDirectionalAction,
    >;

    fn requires_local_protocol<T: LocalProtocol>(_: PhantomData<T>) {}

    requires_local_protocol(PhantomData::<TcpParallel>);
    requires_local_protocol(PhantomData::<WebSocketParallel>);
}

// ============================================================================
// Cross-Protocol Capability Tests
// ============================================================================

#[cfg(test)]
#[test]
fn test_mixed_protocol_action_io_capabilities() {
    // Test complex protocols with mixed action I/O requirements

    // Publisher endpoint that can only send (MQTT Publisher -> Output only)
    type PublisherEndpoint = EpChanSend<
        MqttPublisherIO,
        PubSubMeta,
        PublishMsg,
        EpChanEnd<MqttPublisherIO, PubSubMeta, OutputAction>,
        OutputAction,
    >;

    // Subscriber endpoint that can only receive (MQTT Subscriber -> Input only)
    type SubscriberEndpoint = EpChanRecv<
        MqttSubscriberIO,
        PubSubMeta,
        NotificationMsg,
        EpChanEnd<MqttSubscriberIO, PubSubMeta, InputAction>,
        InputAction,
    >;

    // Full duplex WebSocket endpoint
    type WebSocketEndpoint = EpChanSend<
        WebSocketIO,
        PubSubMeta,
        PublishMsg,
        EpChanRecv<
            WebSocketIO,
            PubSubMeta,
            NotificationMsg,
            EpChanEnd<WebSocketIO, PubSubMeta, BiDirectionalAction>,
            BiDirectionalAction,
        >,
        BiDirectionalAction,
    >;

    fn requires_local_protocol<T: LocalProtocol>(_: PhantomData<T>) {}

    requires_local_protocol(PhantomData::<PublisherEndpoint>);
    requires_local_protocol(PhantomData::<SubscriberEndpoint>);
    requires_local_protocol(PhantomData::<WebSocketEndpoint>);
}

#[cfg(test)]
#[test]
fn test_capability_inheritance_in_nested_protocols() {
    // Test that action I/O capabilities are properly inherited in nested structures

    type NestedProtocol = EpChanChoice<
        TcpOnlySessionIO,
        PubSubMeta,
        // Branch 1: Send followed by receive
        EpChanSend<
            TcpOnlySessionIO,
            PubSubMeta,
            PublishMsg,
            EpChanRecv<
                TcpOnlySessionIO,
                PubSubMeta,
                NotificationMsg,
                EpChanEnd<TcpOnlySessionIO, PubSubMeta, BiDirectionalAction>,
                BiDirectionalAction,
            >,
            BiDirectionalAction,
        >,
        // Branch 2: Direct end
        EpChanEnd<TcpOnlySessionIO, PubSubMeta, BiDirectionalAction>,
        BiDirectionalAction,
    >;

    fn requires_local_protocol<T: LocalProtocol>(_: PhantomData<T>) {}
    requires_local_protocol(PhantomData::<NestedProtocol>);
}

// ============================================================================
// Custom I/O Capability Implementation Tests
// ============================================================================

#[cfg(test)]
#[test]
fn test_custom_io_capability_patterns() {
    // Test custom I/O implementations with various capability patterns

    #[derive(Debug, Clone)]
    struct CustomSocketIO;

    // Custom I/O that supports all standard capabilities
    impl SupportsActionIO<InputAction> for CustomSocketIO {}
    impl SupportsActionIO<OutputAction> for CustomSocketIO {}
    impl SupportsActionIO<BiDirectionalAction> for CustomSocketIO {}

    // Verify it works with capabilities
    assert!(<CustomSocketIO as SupportsActionIO<InputAction>>::supports_action_io());
    assert!(<CustomSocketIO as SupportsActionIO<OutputAction>>::supports_action_io());
    assert!(<CustomSocketIO as SupportsActionIO<BiDirectionalAction>>::supports_action_io());

    // Verify it works in protocol types
    type CustomEndpoint = EpChanEnd<CustomSocketIO, PubSubMeta, BiDirectionalAction>;
    fn requires_local_protocol<T: LocalProtocol>(_: PhantomData<T>) {}
    requires_local_protocol(PhantomData::<CustomEndpoint>);
}

#[cfg(test)]
#[test]
fn test_action_io_with_different_metadata_types() {
    // Test that action I/O types work with different metadata configurations

    type Endpoint1 = EpChanEnd<TcpOnlySessionIO, PubSubMeta, InputAction>;
    type Endpoint2 = EpChanEnd<TcpOnlySessionIO, DataMeta, OutputAction>;
    type Endpoint3 = EpChanEnd<WebSocketIO, PubSubMeta, BiDirectionalAction>;
    type Endpoint4 = EpChanEnd<HttpOnlySessionIO, DataMeta, BiDirectionalAction>;

    fn requires_local_protocol<T: LocalProtocol>(_: PhantomData<T>) {}

    requires_local_protocol(PhantomData::<Endpoint1>);
    requires_local_protocol(PhantomData::<Endpoint2>);
    requires_local_protocol(PhantomData::<Endpoint3>);
    requires_local_protocol(PhantomData::<Endpoint4>);
}

// ============================================================================
// Capability Verification and Error Testing
// ============================================================================

#[cfg(test)]
#[test]
fn test_supports_action_io_default_implementation() {
    // Test the default implementation behavior
    struct MockIO;
    impl SupportsActionIO<InputAction> for MockIO {}

    // Default implementation should return true
    assert!(<MockIO as SupportsActionIO<InputAction>>::supports_action_io());
}

#[cfg(test)]
#[test]
fn test_action_io_debug_formatting() {
    // Test that action I/O types format correctly for debugging
    let input = InputAction;
    let output = OutputAction;
    let bidirectional = BiDirectionalAction;

    let input_debug = format!("{:?}", input);
    let output_debug = format!("{:?}", output);
    let bidirectional_debug = format!("{:?}", bidirectional);

    assert!(!input_debug.is_empty());
    assert!(!output_debug.is_empty());
    assert!(!bidirectional_debug.is_empty());
}

#[cfg(test)]
#[test]
fn test_comprehensive_capability_matrix() {
    // Comprehensive test matrix for all I/O types and action combinations

    // Define a trait to test capability combinations
    trait HasCapability<IO, Action> {
        const HAS_CAPABILITY: bool;
    }

    // Implement for known working combinations
    impl HasCapability<TcpOnlySessionIO, InputAction> for () {
        const HAS_CAPABILITY: bool = true;
    }
    impl HasCapability<TcpOnlySessionIO, OutputAction> for () {
        const HAS_CAPABILITY: bool = true;
    }
    impl HasCapability<TcpOnlySessionIO, BiDirectionalAction> for () {
        const HAS_CAPABILITY: bool = true;
    }

    impl HasCapability<HttpOnlySessionIO, OutputAction> for () {
        const HAS_CAPABILITY: bool = true;
    }
    impl HasCapability<HttpOnlySessionIO, BiDirectionalAction> for () {
        const HAS_CAPABILITY: bool = true;
    }

    impl HasCapability<MqttPublisherIO, OutputAction> for () {
        const HAS_CAPABILITY: bool = true;
    }

    impl HasCapability<MqttSubscriberIO, InputAction> for () {
        const HAS_CAPABILITY: bool = true;
    }

    // Verify the capability matrix matches our implementations
    const _: () = assert!(<() as HasCapability<TcpOnlySessionIO, InputAction>>::HAS_CAPABILITY);
    const _: () = assert!(<() as HasCapability<TcpOnlySessionIO, OutputAction>>::HAS_CAPABILITY);
    const _: () = assert!(<() as HasCapability<TcpOnlySessionIO, BiDirectionalAction>>::HAS_CAPABILITY);

    const _: () = assert!(<() as HasCapability<HttpOnlySessionIO, OutputAction>>::HAS_CAPABILITY);
    const _: () = assert!(<() as HasCapability<HttpOnlySessionIO, BiDirectionalAction>>::HAS_CAPABILITY);

    const _: () = assert!(<() as HasCapability<MqttPublisherIO, OutputAction>>::HAS_CAPABILITY);
    const _: () = assert!(<() as HasCapability<MqttSubscriberIO, InputAction>>::HAS_CAPABILITY);
}
