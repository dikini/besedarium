use super::*;

#[test]
fn test_comm_metadata_creation() {
    let metadata = CommMetadata::new(DefaultChan, RequestLbl);
    assert_eq!(metadata.chan_id, DefaultChan);
    assert_eq!(metadata.msg_lbl, RequestLbl);
}

#[test]
fn test_action_io_support() {
    // Test TcpOnlySessionIO supports all action types
    assert!(<TcpOnlySessionIO as SupportsActionIO<InputAction>>::supports_action_io());
    assert!(<TcpOnlySessionIO as SupportsActionIO<OutputAction>>::supports_action_io());
    assert!(<TcpOnlySessionIO as SupportsActionIO<
        BiDirectionalAction,
    >>::supports_action_io());

    // Test HttpOnlySessionIO supports output and bidirectional
    assert!(<HttpOnlySessionIO as SupportsActionIO<OutputAction>>::supports_action_io());
    assert!(<HttpOnlySessionIO as SupportsActionIO<
        BiDirectionalAction,
    >>::supports_action_io());
}

// Test that example roles implement the Role trait
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TestRole;
impl Role for TestRole {}

#[test]
fn test_role_implementation() {
    let role1 = TestRole;
    let role2 = TestRole;
    assert_eq!(role1, role2);
}

// Test that example messages implement the Message trait
#[derive(Debug, Clone)]
struct TestMessage(String);
impl Message for TestMessage {}

#[test]
fn test_message_implementation() {
    let msg = TestMessage("test".to_string());
    let _clone = msg.clone();
}

// Type safety verification
#[test]
fn test_type_safety() {
    // This should work
    type ExampleMetadata = CommMetadata<DefaultChan, RequestLbl>;
    let meta = ExampleMetadata::new(DefaultChan, RequestLbl);

    // This should provide proper type safety
    fn process_metadata<C: ChanId, L: MsgLbl>(_meta: CommMetadata<C, L>) {
        // Implementation
    }

    process_metadata(meta);
}

#[test]
fn test_action_io_markers() {
    let input = InputAction;
    let output = OutputAction;
    let bidirectional = BiDirectionalAction;

    assert_eq!(input, InputAction);
    assert_eq!(output, OutputAction);
    assert_eq!(bidirectional, BiDirectionalAction);
}
