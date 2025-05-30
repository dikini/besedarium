// src/protocol/test_utils.rs

use crate::protocol::foundation::GlobalProtocol;
use crate::runtime::ExecutionContext; // Corrected import for ExecutionContext

/// A mock protocol for testing purposes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MockProtocol;

// GlobalProtocol requires Send + Sync + 'static + Debug.
// Deriving Debug and Clone. Send and Sync are implicitly implemented for MockProtocol
// as it contains no fields that are not Send/Sync.
// 'static is also true as it has no non-'static lifetimes.
impl GlobalProtocol for MockProtocol {}

// Example of a method that might be needed by ProtocolState,
// to be fleshed out if state.rs requires it.
impl MockProtocol {
    #[allow(dead_code)]
    pub fn is_terminal(&self) -> bool {
        // For a mock, always consider it terminal or non-terminal based on test needs.
        // Let's default to false for now, meaning it can always transition.
        false
    }

    #[allow(dead_code)]
    pub fn can_transition_to<NextP: GlobalProtocol>(&self, _next: &NextP) -> bool {
        // For a mock, always allow transitions or implement specific logic if needed for tests.
        true
    }
}

// Test roles and messages that might be used with MockProtocol in session tests.
// These can be expanded as needed.

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MockRoleA;
impl crate::protocol::foundation::Role for MockRoleA {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MockRoleB;
impl crate::protocol::foundation::Role for MockRoleB {}

#[derive(Clone, Debug, PartialEq, Eq)] // Added PartialEq and Eq for MockMessage as well for consistency
pub struct MockMessage;
// Removed conflicting impl Message for MockMessage as a blanket impl exists in endpoints.rs


// Added MockLocalProtocol for session tests
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MockLocalProtocol {
    // Added a field to store a label, can be set in tests.
    label: String,
    // Added a context field, as LocalProtocol requires it.
    // For mock purposes, it can be a default or configurable context.
    context: ExecutionContext, // Use the imported ExecutionContext
}

impl MockLocalProtocol {
    // Constructor to create a MockLocalProtocol with a specific label and context.
    #[allow(dead_code)]
    pub fn new(label: String, context: ExecutionContext) -> Self { // Use the imported ExecutionContext
        Self { label, context }
    }

    // Default constructor for convenience in tests where specific label/context isn't critical initially.
    #[allow(dead_code)]
    pub fn default() -> Self {
        Self {
            label: "MockLocalProtocolDefaultLabel".to_string(),
            context: ExecutionContext::new("mock_role".to_string()), // Use the imported ExecutionContext
        }
    }
}

impl crate::protocol::foundation::LocalProtocol for MockLocalProtocol {
    fn context(&self) -> &ExecutionContext { // Use the imported ExecutionContext
        &self.context
    }

    fn context_mut(&mut self) -> &mut ExecutionContext { // Use the imported ExecutionContext
        &mut self.context
    }

    fn state_label(&self) -> String {
        self.label.clone()
    }
}
