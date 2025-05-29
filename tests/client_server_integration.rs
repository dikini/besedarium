//! Client-Server Authentication Protocol Integration Test
//!
//! This test demonstrates a realistic client-server authentication protocol
//! using the modern besedarium foundation types.

mod integration_common;

use besedarium::protocol::foundation::*;
use besedarium::protocol::global::*;
use integration_common::{ // Import specific types needed
    AckLbl, AckMsg, Alice, AuthChan, Bob, Charlie, DataChan, DataLbl, DataMsg, LoginLbl, LoginMsg
};
use besedarium::protocol::projection::*; // Keep wildcard for projection traits


#[cfg(test)]
mod client_server_tests {
    use super::*;

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
        let login = LoginMsg("alice".to_string(), "secret".to_string());
        let ack = AckMsg(true, Some("token123".to_string()));
        let data = DataMsg(b"test data".to_vec());

        // Verify messages implement required traits
        fn requires_message<T: Message>(_: T) {}
        requires_message(login);
        requires_message(ack);
        requires_message(data);
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
        type ThreePartyProtocol = TChanSend<
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

        // Verify protocol compiles
        fn requires_global_protocol<T: GlobalProtocol>(_: std::marker::PhantomData<T>) {}
        requires_global_protocol(std::marker::PhantomData::<ThreePartyProtocol>);
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
        // use besedarium::protocol::projection::*; // Already imported at the top level

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
        // use besedarium::protocol::projection::*; // Already imported
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
}
