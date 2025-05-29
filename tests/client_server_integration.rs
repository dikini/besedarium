//! Client-Server Authentication Protocol Integration Test
//!
//! This test demonstrates a realistic client-server authentication protocol
//! using the modern besedarium foundation types.

mod integration_common;

use besedarium::protocol::foundation::*;
use besedarium::protocol::global::*;
use besedarium::protocol::projection::*;
use integration_common::{
    // Import specific types needed
    AckLbl,
    AckMsg,
    Alice,
    AuthChan,
    Bob,
    Charlie,
    Client, // Added Client
    DataChan,
    DataLbl,
    DataMsg,
    Database, // Added Database
    DbChan,   // Added DbChan
    LoginLbl,
    LoginMsg,
    QueryLbl,         // Added QueryLbl
    QueryMsg,         // Added QueryMsg
    ResultLbl,        // Added ResultLbl
    ResultMsg,        // Added ResultMsg
    ServerCommandLbl, // Added for complex data test
    ServerCommandMsg, // Added for complex data test
    Service,          // Added Service
    ServiceChan,      // Added ServiceChan
    UserProfile,      // Added for complex data test
    UserProfileLbl,   // Added for complex data test
    UserProfileMsg,   // Added for complex data test
}; // Keep wildcard for projection traits

#[cfg(test)]
mod client_server_tests {
    use super::*;
    use integration_common::{OrderDetails, ServerCommand}; // Ensure these are in scope

    /// Simple Login Protocol: Client → Server (login) → Client (ack) → End
    type LoginProtocol = TChanSend<
        Alice,    // Sender: Client (Alice)
        Bob,      // Receiver: Server (Bob)
        AuthChan, // Channel: Auth channel
        LoginLbl, // Message label: Login
        LoginMsg, // Message: Login credentials
        TChanSend<
            // Continuation: Server responds
            Bob,                                             // Sender: Server (Bob)
            Alice,                                           // Receiver: Client (Alice)
            AuthChan,                                        // Channel: Auth channel
            AckLbl,                                          // Message label: Ack
            AckMsg,                                          // Message: Acknowledgment
            TChanEnd<AuthChan, AckLbl, BiDirectionalAction>, // End protocol
            BiDirectionalAction,
        >,
        BiDirectionalAction,
    >;

    #[test]
    fn test_login_protocol_compilation() {
        // Test that our login protocol compiles correctly
        fn requires_global_protocol<T: GlobalProtocol>(_: std::marker::PhantomData<T>) {}
        requires_global_protocol(std::marker::PhantomData::<LoginProtocol>);
    }

    #[test]
    fn test_protocol_types_are_valid() {
        // Test that fundamental types implement the correct traits
        fn requires_role<T: Role>(_: std::marker::PhantomData<T>) {}
        fn requires_chan_id<T: ChanId>(_: std::marker::PhantomData<T>) {}
        fn requires_msg_lbl<T: MsgLbl>(_: std::marker::PhantomData<T>) {}
        fn requires_message<T: Message>(_: std::marker::PhantomData<T>) {}
        fn requires_action_io<T: ActionIOTMarker>(_: std::marker::PhantomData<T>) {}

        // Test roles
        requires_role(std::marker::PhantomData::<Alice>);
        requires_role(std::marker::PhantomData::<Bob>);
        requires_role(std::marker::PhantomData::<Charlie>);

        // Test channels
        requires_chan_id(std::marker::PhantomData::<AuthChan>);
        requires_chan_id(std::marker::PhantomData::<DataChan>);

        // Test message labels
        requires_msg_lbl(std::marker::PhantomData::<LoginLbl>);
        requires_msg_lbl(std::marker::PhantomData::<AckLbl>);
        requires_msg_lbl(std::marker::PhantomData::<DataLbl>);

        // Test messages
        requires_message(std::marker::PhantomData::<LoginMsg>);
        requires_message(std::marker::PhantomData::<AckMsg>);
        requires_message(std::marker::PhantomData::<DataMsg>);

        // Test action IO markers
        requires_action_io(std::marker::PhantomData::<BiDirectionalAction>);
    }

    #[test]
    fn test_simple_protocol_structures() {
        // Test simple TChanEnd compiles
        type SimpleEnd = TChanEnd<AuthChan, LoginLbl, BiDirectionalAction>;
        fn requires_global_protocol<T: GlobalProtocol>(_: std::marker::PhantomData<T>) {}
        requires_global_protocol(std::marker::PhantomData::<SimpleEnd>);

        // Test simple TChanSend compiles
        type SimpleSend =
            TChanSend<Alice, Bob, AuthChan, LoginLbl, LoginMsg, SimpleEnd, BiDirectionalAction>;
        requires_global_protocol(std::marker::PhantomData::<SimpleSend>);
    }

    #[test]
    fn test_message_creation() {
        // Test that messages can be created and used
        let login_msg = LoginMsg("alice".to_string(), "secret".to_string());
        let ack_msg = AckMsg(true, Some("token123".to_string()));
        let data_msg = DataMsg(b"test data".to_vec());

        // Verify messages implement required traits
        fn requires_message<T: Message>(_: T) {}
        requires_message(login_msg);
        requires_message(ack_msg);
        requires_message(data_msg);
    }

    #[test]
    fn test_metadata_integration() {
        // Test that metadata types integrate properly with protocol types
        let auth_meta = CommMetadata::new(AuthChan, LoginLbl);
        let data_meta = CommMetadata::new(DataChan, DataLbl);

        // Test metadata satisfies required traits
        fn requires_metadata<T: Metadata>(_: T) {}
        requires_metadata(auth_meta);
        requires_metadata(data_meta);
    }

    #[test]
    fn test_protocol_duality() {
        // Test that our login protocol has proper duality
        // This verifies the dual generation works correctly

        // For now, just test that dual protocols can be constructed
        // The actual duality verification will be added when the system is complete

        // Define the server's perspective protocol (dual of client protocol)
        type ServerProtocol = TChanRecv<
            Alice,    // Receiver: Server receives from Client
            Bob,      // Sender: (from server perspective)
            AuthChan, // Channel: Auth channel
            LoginLbl, // Message label: Login
            LoginMsg, // Message: Login credentials
            TChanRecv<
                // Continuation: Server sends response
                Bob,      // Receiver: (from server perspective)
                Alice,    // Sender: Server sends to Client
                AuthChan, // Channel: Auth channel
                AckLbl,   // Message label: Ack
                AckMsg,   // Message: Acknowledgment
                TChanEnd<AuthChan, AckLbl, BiDirectionalAction>, // End protocol
                BiDirectionalAction,
            >,
            BiDirectionalAction,
        >;

        // Verify both protocols are valid
        fn requires_global_protocol<T: GlobalProtocol>(_: std::marker::PhantomData<T>) {}
        requires_global_protocol(std::marker::PhantomData::<LoginProtocol>);
        requires_global_protocol(std::marker::PhantomData::<ServerProtocol>);

        // TODO: When IsDual implementations are complete, add duality verification here
        // For now, just test compilation succeeds
    }

    #[test]
    fn test_multi_party_protocol() {
        // Test a more complex protocol involving three parties
        // Original ThreePartyProtocol involving Alice, Bob, Charlie remains for now
        type OriginalThreePartyProtocol = TChanSend<
            Alice, // Client sends to Server
            Bob,
            AuthChan,
            LoginLbl,
            LoginMsg,
            TChanSend<
                // Server forwards to Database
                Bob,
                Charlie,
                DataChan,
                DataLbl,
                DataMsg,
                TChanRecv<
                    // Database responds to Server
                    Charlie,
                    Bob,
                    DataChan,
                    AckLbl,
                    AckMsg,
                    TChanSend<
                        // Server responds to Client
                        Bob,
                        Alice,
                        AuthChan,
                        AckLbl,
                        AckMsg,
                        TChanEnd<AuthChan, AckLbl, BiDirectionalAction>,
                        BiDirectionalAction,
                    >,
                    BiDirectionalAction,
                >,
                BiDirectionalAction,
            >,
            BiDirectionalAction,
        >;

        // Verify original protocol compiles
        fn requires_global_protocol<T: GlobalProtocol>(_: std::marker::PhantomData<T>) {}
        requires_global_protocol(std::marker::PhantomData::<OriginalThreePartyProtocol>);

        // New Query Protocol: Client -> Service -> Database -> Service -> Client
        type QueryServiceProtocol = TChanSend<
            Client,
            Service,
            ServiceChan,
            QueryLbl,
            QueryMsg,
            TChanSend<
                Service,
                Database,
                DbChan,
                QueryLbl,
                QueryMsg, // Service forwards the same query
                TChanRecv<
                    Database,
                    Service,
                    DbChan,
                    ResultLbl,
                    ResultMsg,
                    TChanRecv<
                        Service,
                        Client,
                        ServiceChan,
                        ResultLbl,
                        ResultMsg, // Service forwards the result
                        TChanEnd<ServiceChan, ResultLbl, BiDirectionalAction>,
                        BiDirectionalAction,
                    >,
                    BiDirectionalAction,
                >,
                BiDirectionalAction,
            >,
            BiDirectionalAction,
        >;

        // Verify new query protocol compiles
        requires_global_protocol(std::marker::PhantomData::<QueryServiceProtocol>);

        // Test projection for the new QueryServiceProtocol
        type ClientEndpointQuery = <() as Project<QueryServiceProtocol, Client>>::Output;
        type ServiceEndpointQuery = <() as Project<QueryServiceProtocol, Service>>::Output;
        type DatabaseEndpointQuery = <() as Project<QueryServiceProtocol, Database>>::Output;

        // Verify projections are valid local protocols
        fn requires_local_protocol<T: LocalProtocol>(_: std::marker::PhantomData<T>) {}
        requires_local_protocol(std::marker::PhantomData::<ClientEndpointQuery>);
        requires_local_protocol(std::marker::PhantomData::<ServiceEndpointQuery>);
        requires_local_protocol(std::marker::PhantomData::<DatabaseEndpointQuery>);

        // Optional: Add assertions to check the structure of projected types if needed
        // This can be useful for debugging or deep understanding but makes tests verbose.
        // Example (conceptual, actual types might be complex and require helper assertions):
        // assert_eq!(std::any::type_name::<ClientEndpointQuery>(), "...");
    }

    #[test]
    fn test_choice_protocol() {
        // Test a protocol with choices for branching behavior
        type ChoiceProtocol = TChanSend<
            Alice,
            Bob,
            AuthChan,
            LoginLbl,
            LoginMsg,
            TChanChoice<
                Bob, // Bob makes the choice
                AuthChan,
                AckLbl,
                // Success branch: Send acknowledgment
                TChanSend<
                    Bob,
                    Alice,
                    AuthChan,
                    AckLbl,
                    AckMsg,
                    TChanEnd<AuthChan, AckLbl, BiDirectionalAction>,
                    BiDirectionalAction,
                >,
                // Failure branch: End immediately
                TChanEnd<AuthChan, LoginLbl, BiDirectionalAction>,
                BiDirectionalAction,
            >,
            BiDirectionalAction,
        >;

        // Verify protocol compiles
        fn requires_global_protocol<T: GlobalProtocol>(_: std::marker::PhantomData<T>) {}
        requires_global_protocol(std::marker::PhantomData::<ChoiceProtocol>);
    }

    #[test]
    fn test_protocol_projection() {
        // Test that protocols can be projected to local endpoints

        // Test projection for Alice's role in the login protocol
        type AliceEndpoint = <() as Project<LoginProtocol, Alice>>::Output;
        type BobEndpoint = <() as Project<LoginProtocol, Bob>>::Output;

        // Verify projections are valid local protocols
        fn requires_local_protocol<T: LocalProtocol>(_: std::marker::PhantomData<T>) {}
        requires_local_protocol(std::marker::PhantomData::<AliceEndpoint>);
        requires_local_protocol(std::marker::PhantomData::<BobEndpoint>);
    }

    #[test]
    fn test_comprehensive_protocol_system() {
        // Integration test that verifies the entire protocol type system works together

        // 1. Define a comprehensive protocol
        type ComprehensiveProtocol = TChanSend<
            Alice,
            Bob,
            AuthChan,
            LoginLbl,
            LoginMsg,
            TChanChoice<
                Bob,
                AuthChan,
                AckLbl,
                TChanSend<
                    Bob,
                    Alice,
                    AuthChan,
                    AckLbl,
                    AckMsg,
                    TChanSend<
                        Alice,
                        Charlie,
                        DataChan,
                        DataLbl,
                        DataMsg,
                        TChanEnd<DataChan, DataLbl, BiDirectionalAction>,
                        BiDirectionalAction,
                    >,
                    BiDirectionalAction,
                >,
                TChanEnd<AuthChan, LoginLbl, BiDirectionalAction>,
                BiDirectionalAction,
            >,
            BiDirectionalAction,
        >;

        // 2. Verify it's a valid global protocol
        fn requires_global_protocol<T: GlobalProtocol>(_: std::marker::PhantomData<T>) {}
        requires_global_protocol(std::marker::PhantomData::<ComprehensiveProtocol>);

        // 3. Verify projections work for all roles
        type AliceProj = <() as Project<ComprehensiveProtocol, Alice>>::Output;
        type BobProj = <() as Project<ComprehensiveProtocol, Bob>>::Output;
        type CharlieProj = <() as Project<ComprehensiveProtocol, Charlie>>::Output;

        // Verify projections are valid local protocols
        fn requires_local_protocol<T: LocalProtocol>(_: std::marker::PhantomData<T>) {}
        requires_local_protocol(std::marker::PhantomData::<AliceProj>);
        requires_local_protocol(std::marker::PhantomData::<BobProj>);
        requires_local_protocol(std::marker::PhantomData::<CharlieProj>);
    }

    #[test]
    fn test_complex_protocol_message_instantiation() {
        // Test instantiation of messages used in complex protocols
        let login_msg = LoginMsg("alice".to_string(), "secret123".to_string());
        let ack_msg = AckMsg(true, Some("session_abc123".to_string()));
        let data_msg = DataMsg(b"sensitive user data".to_vec());

        // Verify messages implement required traits
        fn requires_message<T: Message>(_: T) {}
        requires_message(login_msg);
        requires_message(ack_msg);
        requires_message(data_msg);
    }

    // --- Tests for Complex Data Serialization ---

    #[test]
    fn test_complex_data_exchange_user_profile() {
        // Protocol: Alice sends UserProfileMsg to Bob, Bob responds with AckMsg
        type UserProfileExchangeProtocol = TChanSend<
            Alice,          // Sender
            Bob,            // Receiver
            DataChan,       // Channel
            UserProfileLbl, // Message Label
            UserProfileMsg, // Message Type
            TChanSend<
                Bob,      // Sender
                Alice,    // Receiver
                DataChan, // Channel
                AckLbl,   // Message Label
                AckMsg,   // Message Type
                TChanEnd<DataChan, AckLbl, BiDirectionalAction>,
                BiDirectionalAction,
            >,
            BiDirectionalAction,
        >;

        // Verify global protocol compilation
        fn requires_global_protocol<T: GlobalProtocol>(_: std::marker::PhantomData<T>) {}
        requires_global_protocol(std::marker::PhantomData::<UserProfileExchangeProtocol>);

        // Verify projection to local protocols
        type AliceLocal = <() as Project<UserProfileExchangeProtocol, Alice>>::Output;
        type BobLocal = <() as Project<UserProfileExchangeProtocol, Bob>>::Output;

        fn requires_local_protocol<T: LocalProtocol>(_: std::marker::PhantomData<T>) {}
        requires_local_protocol(std::marker::PhantomData::<AliceLocal>);
        requires_local_protocol(std::marker::PhantomData::<BobLocal>);

        // Simulate execution (conceptual - actual execution requires runtime)
        let user_profile_data = UserProfile {
            user_id: 101,
            username: "complex_user".to_string(),
            email: Some("complex@example.com".to_string()),
            preferences: vec![("lang".to_string(), "en".to_string())],
            aliases: vec!["tester_one".to_string(), "dev_user".to_string()],
        };
        let sent_msg = UserProfileMsg(user_profile_data.clone());
        let ack_msg = AckMsg(true, Some("session_complex_profile".to_string()));

        // In a real scenario, you'd send `sent_msg` and receive `ack_msg`
        // Here, we just assert that the types are correct and data can be constructed.
        assert_eq!(sent_msg.0, user_profile_data);
        assert!(ack_msg.0);
    }

    #[test]
    fn test_complex_data_exchange_server_command() {
        // Protocol: Client sends ServerCommandMsg to Service, Service responds with AckMsg
        type ServerCommandExchangeProtocol = TChanSend<
            Client,           // Sender
            Service,          // Receiver
            ServiceChan,      // Channel
            ServerCommandLbl, // Message Label
            ServerCommandMsg, // Message Type
            TChanSend<
                Service,     // Sender
                Client,      // Receiver
                ServiceChan, // Channel
                AckLbl,      // Message Label
                AckMsg,      // Message Type
                TChanEnd<ServiceChan, AckLbl, BiDirectionalAction>,
                BiDirectionalAction,
            >,
            BiDirectionalAction,
        >;

        // Verify global protocol compilation
        fn requires_global_protocol<T: GlobalProtocol>(_: std::marker::PhantomData<T>) {}
        requires_global_protocol(std::marker::PhantomData::<ServerCommandExchangeProtocol>);

        // Verify projection to local protocols
        type ClientLocal = <() as Project<ServerCommandExchangeProtocol, Client>>::Output;
        type ServiceLocal = <() as Project<ServerCommandExchangeProtocol, Service>>::Output;

        fn requires_local_protocol<T: LocalProtocol>(_: std::marker::PhantomData<T>) {}
        requires_local_protocol(std::marker::PhantomData::<ClientLocal>);
        requires_local_protocol(std::marker::PhantomData::<ServiceLocal>);

        // Simulate execution with ServerCommand::StoreProfile
        let user_profile_data = UserProfile {
            user_id: 202,
            username: "command_user".to_string(),
            email: None,
            preferences: vec![],
            aliases: vec!["cmd_alias".to_string()],
        };
        let command_store_profile = ServerCommand::StoreProfile(user_profile_data.clone());
        let sent_cmd_msg_profile = ServerCommandMsg(command_store_profile.clone());
        let ack_msg_profile = AckMsg(true, Some("session_store_profile".to_string()));

        assert_eq!(sent_cmd_msg_profile.0, command_store_profile);
        assert!(ack_msg_profile.0);

        // Simulate execution with ServerCommand::ProcessOrder
        let order_details_data = OrderDetails {
            item_id: "item789".to_string(),
            quantity: 5,
            notes: Some("Urgent delivery".to_string()),
        };
        let command_process_order = ServerCommand::ProcessOrder(order_details_data.clone());
        let sent_cmd_msg_order = ServerCommandMsg(command_process_order.clone());
        let ack_msg_order = AckMsg(true, Some("session_process_order".to_string()));

        assert_eq!(sent_cmd_msg_order.0, command_process_order);
        assert!(ack_msg_order.0);

        // Simulate execution with ServerCommand::GetStatus
        let command_get_status = ServerCommand::GetStatus;
        let sent_cmd_msg_status = ServerCommandMsg(command_get_status.clone());
        let ack_msg_status = AckMsg(true, Some("session_get_status".to_string()));

        assert_eq!(sent_cmd_msg_status.0, command_get_status);
        assert!(ack_msg_status.0);
    }

    #[test]
    fn test_runtime_placeholder() {
        // This test is a placeholder for future runtime integration tests.
        // It ensures that the test suite can include tests that might involve
        // async runtimes or other complex setups without causing issues now.
        // Placeholder for actual runtime execution simulation
    }

    // TODO: Add more tests for:
    // - Error cases and recovery
    // - Performance under load
    // - Security aspects (e.g., authentication, authorization)
    // - Edge cases in protocol usage
}
