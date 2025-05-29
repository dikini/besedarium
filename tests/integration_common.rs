//! Integration Tests for Besedarium Session Types Library
//!
//! This module contains integration tests that demonstrate complex protocol scenarios
//! using the modern foundation types and architecture.

use besedarium::{
    // Grouped imports from besedarium crate
    BiDirectionalAction,
    ChanId as ChanIdTrait,
    HasDual,
    InputAction,
    Message as MessageTrait, // Renamed to avoid conflict
    MsgLbl as MsgLblTrait,   // Renamed to avoid conflict
    OutputAction,
    ProtocolLabel,
    Role as RoleTrait, // Renamed to avoid conflict
    SessionType,
    SupportsActionIO,
};

// Helper for RoleEq
use besedarium::protocol::projection::helpers::{False, RoleEq};

// --- Role Definitions ---
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Alice;
impl RoleTrait for Alice {}
impl SupportsActionIO<InputAction> for Alice {}
impl SupportsActionIO<OutputAction> for Alice {}
impl SupportsActionIO<BiDirectionalAction> for Alice {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bob;
impl RoleTrait for Bob {}
impl SupportsActionIO<InputAction> for Bob {}
impl SupportsActionIO<OutputAction> for Bob {}
impl SupportsActionIO<BiDirectionalAction> for Bob {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Charlie; // Added for potential multi-party scenarios
impl RoleTrait for Charlie {}
impl SupportsActionIO<InputAction> for Charlie {}
impl SupportsActionIO<OutputAction> for Charlie {}
impl SupportsActionIO<BiDirectionalAction> for Charlie {}

// Roles for Multi-Party Scenario
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Buyer;
impl RoleTrait for Buyer {}
impl SupportsActionIO<InputAction> for Buyer {}
impl SupportsActionIO<OutputAction> for Buyer {}
impl SupportsActionIO<BiDirectionalAction> for Buyer {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seller1;
impl RoleTrait for Seller1 {}
impl SupportsActionIO<InputAction> for Seller1 {}
impl SupportsActionIO<OutputAction> for Seller1 {}
impl SupportsActionIO<BiDirectionalAction> for Seller1 {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seller2;
impl RoleTrait for Seller2 {}
impl SupportsActionIO<InputAction> for Seller2 {}
impl SupportsActionIO<OutputAction> for Seller2 {}
impl SupportsActionIO<BiDirectionalAction> for Seller2 {}

// New Roles for Query Protocol
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Client;
impl RoleTrait for Client {}
impl SupportsActionIO<InputAction> for Client {}
impl SupportsActionIO<OutputAction> for Client {}
impl SupportsActionIO<BiDirectionalAction> for Client {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Service;
impl RoleTrait for Service {}
impl SupportsActionIO<InputAction> for Service {}
impl SupportsActionIO<OutputAction> for Service {}
impl SupportsActionIO<BiDirectionalAction> for Service {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Database;
impl RoleTrait for Database {}
impl SupportsActionIO<InputAction> for Database {}
impl SupportsActionIO<OutputAction> for Database {}
impl SupportsActionIO<BiDirectionalAction> for Database {}

// --- RoleEq Implementations (Non-Reflexive) ---
// Alice vs Others
impl RoleEq<Bob> for Alice {
    type Output = False;
}
impl RoleEq<Charlie> for Alice {
    type Output = False;
}
impl RoleEq<Buyer> for Alice {
    type Output = False;
}
impl RoleEq<Seller1> for Alice {
    type Output = False;
}
impl RoleEq<Seller2> for Alice {
    type Output = False;
}

// Bob vs Others
impl RoleEq<Alice> for Bob {
    type Output = False;
}
impl RoleEq<Charlie> for Bob {
    type Output = False;
}
impl RoleEq<Buyer> for Bob {
    type Output = False;
}
impl RoleEq<Seller1> for Bob {
    type Output = False;
}
impl RoleEq<Seller2> for Bob {
    type Output = False;
}

// Charlie vs Others
impl RoleEq<Alice> for Charlie {
    type Output = False;
}
impl RoleEq<Bob> for Charlie {
    type Output = False;
}
impl RoleEq<Buyer> for Charlie {
    type Output = False;
}
impl RoleEq<Seller1> for Charlie {
    type Output = False;
}
impl RoleEq<Seller2> for Charlie {
    type Output = False;
}

// Buyer vs Others
impl RoleEq<Alice> for Buyer {
    type Output = False;
}
impl RoleEq<Bob> for Buyer {
    type Output = False;
}
impl RoleEq<Charlie> for Buyer {
    type Output = False;
}
impl RoleEq<Seller1> for Buyer {
    type Output = False;
}
impl RoleEq<Seller2> for Buyer {
    type Output = False;
}

// Seller1 vs Others
impl RoleEq<Alice> for Seller1 {
    type Output = False;
}
impl RoleEq<Bob> for Seller1 {
    type Output = False;
}
impl RoleEq<Charlie> for Seller1 {
    type Output = False;
}
impl RoleEq<Buyer> for Seller1 {
    type Output = False;
}
impl RoleEq<Seller2> for Seller1 {
    type Output = False;
}

// Seller2 vs Others
impl RoleEq<Alice> for Seller2 {
    type Output = False;
}
impl RoleEq<Bob> for Seller2 {
    type Output = False;
}
impl RoleEq<Charlie> for Seller2 {
    type Output = False;
}
impl RoleEq<Buyer> for Seller2 {
    type Output = False;
}
impl RoleEq<Seller1> for Seller2 {
    type Output = False;
}

// Client vs Others
impl RoleEq<Alice> for Client {
    type Output = False;
}
impl RoleEq<Bob> for Client {
    type Output = False;
}
impl RoleEq<Charlie> for Client {
    type Output = False;
}
impl RoleEq<Buyer> for Client {
    type Output = False;
}
impl RoleEq<Seller1> for Client {
    type Output = False;
}
impl RoleEq<Seller2> for Client {
    type Output = False;
}
impl RoleEq<Service> for Client {
    type Output = False;
}
impl RoleEq<Database> for Client {
    type Output = False;
}

// Service vs Others
impl RoleEq<Alice> for Service {
    type Output = False;
}
impl RoleEq<Bob> for Service {
    type Output = False;
}
impl RoleEq<Charlie> for Service {
    type Output = False;
}
impl RoleEq<Buyer> for Service {
    type Output = False;
}
impl RoleEq<Seller1> for Service {
    type Output = False;
}
impl RoleEq<Seller2> for Service {
    type Output = False;
}
impl RoleEq<Client> for Service {
    type Output = False;
}
impl RoleEq<Database> for Service {
    type Output = False;
}

// Database vs Others
impl RoleEq<Alice> for Database {
    type Output = False;
}
impl RoleEq<Bob> for Database {
    type Output = False;
}
impl RoleEq<Charlie> for Database {
    type Output = False;
}
impl RoleEq<Buyer> for Database {
    type Output = False;
}
impl RoleEq<Seller1> for Database {
    type Output = False;
}
impl RoleEq<Seller2> for Database {
    type Output = False;
}
impl RoleEq<Client> for Database {
    type Output = False;
}
impl RoleEq<Service> for Database {
    type Output = False;
}

// Channels
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthChan;
impl ChanIdTrait for AuthChan {}
impl SessionType for AuthChan {} // Assuming channel types might need to be SessionTypes for HasDual
impl HasDual for AuthChan {
    type Dual = AuthChan;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderChan;
impl ChanIdTrait for OrderChan {}
impl SessionType for OrderChan {}
impl HasDual for OrderChan {
    type Dual = OrderChan;
}

// Channels for multi-party
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QueryChanS1; // Buyer to Seller1
impl ChanIdTrait for QueryChanS1 {}
impl SessionType for QueryChanS1 {}
impl HasDual for QueryChanS1 {
    type Dual = QueryChanS1;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QueryChanS2; // Buyer to Seller2
impl ChanIdTrait for QueryChanS2 {}
impl SessionType for QueryChanS2 {}
impl HasDual for QueryChanS2 {
    type Dual = QueryChanS2;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderChanS1; // Buyer to Seller1
impl ChanIdTrait for OrderChanS1 {}
impl SessionType for OrderChanS1 {}
impl HasDual for OrderChanS1 {
    type Dual = OrderChanS1;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderChanS2; // Buyer to Seller2
impl ChanIdTrait for OrderChanS2 {}
impl SessionType for OrderChanS2 {}
impl HasDual for OrderChanS2 {
    type Dual = OrderChanS2;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataChan;
impl ChanIdTrait for DataChan {}
impl SessionType for DataChan {}
impl HasDual for DataChan {
    type Dual = DataChan;
}

// New Channels for Query Protocol
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServiceChan; // Client to Service
impl ChanIdTrait for ServiceChan {}
impl SessionType for ServiceChan {}
impl HasDual for ServiceChan {
    type Dual = ServiceChan;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DbChan; // Service to Database
impl ChanIdTrait for DbChan {}
impl SessionType for DbChan {}
impl HasDual for DbChan {
    type Dual = DbChan;
}

// Labels (implementing MsgLblTrait and ProtocolLabel)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LLogin;
impl MsgLblTrait for LLogin {}
impl ProtocolLabel for LLogin {}
impl SessionType for LLogin {} // If labels are used in contexts requiring SessionType for HasDual
impl HasDual for LLogin {
    type Dual = LLogin;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LAck;
impl MsgLblTrait for LAck {}
impl ProtocolLabel for LAck {}
impl SessionType for LAck {}
impl HasDual for LAck {
    type Dual = LAck;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LQuoteRequest;
impl MsgLblTrait for LQuoteRequest {}
impl ProtocolLabel for LQuoteRequest {}
impl SessionType for LQuoteRequest {}
impl HasDual for LQuoteRequest {
    type Dual = LQuoteRequest;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LQuoteResponse;
impl MsgLblTrait for LQuoteResponse {}
impl ProtocolLabel for LQuoteResponse {}
impl SessionType for LQuoteResponse {}
impl HasDual for LQuoteResponse {
    type Dual = LQuoteResponse;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoginLbl;
impl MsgLblTrait for LoginLbl {}
impl ProtocolLabel for LoginLbl {}
impl SessionType for LoginLbl {}
impl HasDual for LoginLbl {
    type Dual = LoginLbl;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AckLbl;
impl MsgLblTrait for AckLbl {}
impl ProtocolLabel for AckLbl {}
impl SessionType for AckLbl {}
impl HasDual for AckLbl {
    type Dual = AckLbl;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataLbl;
impl MsgLblTrait for DataLbl {}

// New Message Labels for Query Protocol
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QueryLbl;
impl MsgLblTrait for QueryLbl {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResultLbl;
impl MsgLblTrait for ResultLbl {}

// New Message Labels for Complex Data Types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserProfileLbl;
impl MsgLblTrait for UserProfileLbl {}
impl ProtocolLabel for UserProfileLbl {}
impl SessionType for UserProfileLbl {}
impl HasDual for UserProfileLbl {
    type Dual = UserProfileLbl;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderDetailsLbl;
impl MsgLblTrait for OrderDetailsLbl {}
impl ProtocolLabel for OrderDetailsLbl {}
impl SessionType for OrderDetailsLbl {}
impl HasDual for OrderDetailsLbl {
    type Dual = OrderDetailsLbl;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServerCommandLbl;
impl MsgLblTrait for ServerCommandLbl {}
impl ProtocolLabel for ServerCommandLbl {}
impl SessionType for ServerCommandLbl {}
impl HasDual for ServerCommandLbl {
    type Dual = ServerCommandLbl;
}

// --- Complex Data Structures for Serialization Tests ---
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserProfile {
    pub user_id: u64,
    pub username: String,
    pub email: Option<String>,
    pub preferences: Vec<(String, String)>, // Using Vec of tuples as a simpler alternative to HashMap for Eq/Hash
    pub aliases: Vec<String>,
}
impl MessageTrait for UserProfile {}
impl SessionType for UserProfile {} // Assuming complex types might need to be SessionTypes for HasDual
impl HasDual for UserProfile {
    type Dual = UserProfile; // Simplistic dual for data types
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderDetails {
    pub item_id: String,
    pub quantity: u32,
    pub notes: Option<String>,
}
impl MessageTrait for OrderDetails {}
impl SessionType for OrderDetails {}
impl HasDual for OrderDetails {
    type Dual = OrderDetails;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerCommand {
    StoreProfile(UserProfile),
    ProcessOrder(OrderDetails),
    GetStatus, // A variant without data
}
impl MessageTrait for ServerCommand {}
impl SessionType for ServerCommand {}
impl HasDual for ServerCommand {
    type Dual = ServerCommand;
}

// --- New Message Types Wrapping Complex Data ---
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserProfileMsg(pub UserProfile);
impl MessageTrait for UserProfileMsg {}
impl SessionType for UserProfileMsg {}
impl HasDual for UserProfileMsg {
    type Dual = UserProfileMsg;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderDetailsMsg(pub OrderDetails);
impl MessageTrait for OrderDetailsMsg {}
impl SessionType for OrderDetailsMsg {}
impl HasDual for OrderDetailsMsg {
    type Dual = OrderDetailsMsg;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerCommandMsg(pub ServerCommand);
impl MessageTrait for ServerCommandMsg {}
impl SessionType for ServerCommandMsg {}
impl HasDual for ServerCommandMsg {
    type Dual = ServerCommandMsg;
}

// --- Message Definitions ---
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuoteRequestMsg {
    // Changed to struct with named field
    pub item_id: String,
}
impl MessageTrait for QuoteRequestMsg {}
impl SessionType for QuoteRequestMsg {}
impl HasDual for QuoteRequestMsg {
    type Dual = QuoteRequestMsg;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuoteResponseMsg {
    pub price: u64,
}
impl MessageTrait for QuoteResponseMsg {}
impl SessionType for QuoteResponseMsg {}
impl HasDual for QuoteResponseMsg {
    type Dual = QuoteResponseMsg;
}

// Messages for Client-Server Protocol
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginMsg(pub String, pub String); // username, password
impl MessageTrait for LoginMsg {}
impl SessionType for LoginMsg {}
impl HasDual for LoginMsg {
    type Dual = LoginMsg;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AckMsg(pub bool, pub Option<String>); // success, session_token
impl MessageTrait for AckMsg {}
impl SessionType for AckMsg {}
impl HasDual for AckMsg {
    type Dual = AckMsg;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataMsg(pub Vec<u8>); // payload
impl MessageTrait for DataMsg {}

// New Messages for Query Protocol
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QueryMsg(pub String);
impl MessageTrait for QueryMsg {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResultMsg(pub String);
impl MessageTrait for ResultMsg {}

// Helper types for metadata (if needed, though CommMetadata is generic)
// Commented out as per compiler warning
/*
pub fn auth_login_meta() -> CommMetadata<AuthChan, LLogin> {
    CommMetadata::new(AuthChan, LLogin)
}

pub fn auth_ack_meta() -> CommMetadata<AuthChan, LAck> {
    CommMetadata::new(AuthChan, LAck)
}
*/

// Basic test to ensure types compile (will be expanded with actual protocol tests)
#[cfg(test)]
mod tests {
    use super::*;
    use besedarium::Message as MessageTrait; // Ensure correct Message trait is in scope

    #[test]
    fn test_role_definitions() {
        // Test that role types can be instantiated (implicitly tested by compilation)
        let _alice = Alice;
        let _bob = Bob;
        let _charlie = Charlie;
        let _buyer = Buyer;
        let _seller1 = Seller1;
        let _seller2 = Seller2;
        let _client = Client;
        let _service = Service;
        let _database = Database;
        assert!(true); // Placeholder assertion
    }

    #[test]
    fn test_channel_definitions() {
        // Test that channel types can be instantiated
        let _auth_chan = AuthChan;
        let _order_chan = OrderChan;
        let _data_chan = DataChan;
        let _query_chan_s1 = QueryChanS1;
        let _query_chan_s2 = QueryChanS2;
        let _order_chan_s1 = OrderChanS1;
        let _order_chan_s2 = OrderChanS2;
        let _service_chan = ServiceChan;
        let _db_chan = DbChan;
        assert!(true); // Placeholder assertion
    }

    #[test]
    fn test_label_definitions() {
        // Test that label types can be instantiated
        let _l_login = LLogin;
        let _l_ack = LAck;
        let _l_quote_request = LQuoteRequest;
        let _l_quote_response = LQuoteResponse;
        let _login_lbl = LoginLbl;
        let _ack_lbl = AckLbl;
        let _data_lbl = DataLbl;
        let _query_lbl = QueryLbl;
        let _result_lbl = ResultLbl;
        // Add new labels
        let _user_profile_lbl = UserProfileLbl;
        let _order_details_lbl = OrderDetailsLbl;
        let _server_command_lbl = ServerCommandLbl;
        assert!(true); // Placeholder assertion
    }

    #[test]
    fn test_message_definitions() {
        // Test that message types can be instantiated
        let _login_msg = LoginMsg("user".to_string(), "pass".to_string());
        let _ack_msg = AckMsg(true, Some("token".to_string()));
        let _data_msg = DataMsg(vec![1, 2, 3]);
        let _quote_request_msg = QuoteRequestMsg {
            item_id: "item123".to_string(),
        };
        let _quote_response_msg = QuoteResponseMsg { price: 100 };
        let _query_msg = QueryMsg("SELECT * FROM users".to_string());
        let _result_msg = ResultMsg("user_id: 1, name: Alice".to_string());

        // New complex messages
        let user_profile_data = UserProfile {
            user_id: 1,
            username: "testuser".to_string(),
            email: Some("test@example.com".to_string()),
            preferences: vec![("theme".to_string(), "dark".to_string())],
            aliases: vec!["tester".to_string()],
        };
        let _user_profile_msg = UserProfileMsg(user_profile_data.clone());

        let order_details_data = OrderDetails {
            item_id: "item001".to_string(),
            quantity: 2,
            notes: Some("Gift wrap".to_string()),
        };
        let _order_details_msg = OrderDetailsMsg(order_details_data.clone());

        let _server_command_msg_profile =
            ServerCommandMsg(ServerCommand::StoreProfile(user_profile_data.clone()));
        let _server_command_msg_order =
            ServerCommandMsg(ServerCommand::ProcessOrder(order_details_data.clone()));
        let _server_command_msg_status = ServerCommandMsg(ServerCommand::GetStatus);

        // Verify messages implement required traits
        fn requires_message<T: MessageTrait>(_: T) {}
        requires_message(_login_msg.clone());
        requires_message(_ack_msg.clone());
        requires_message(_data_msg.clone());
        requires_message(_quote_request_msg.clone());
        requires_message(_quote_response_msg.clone());
        requires_message(_query_msg.clone());
        requires_message(_result_msg.clone());
        requires_message(_user_profile_msg.clone());
        requires_message(_order_details_msg.clone());
        requires_message(_server_command_msg_profile.clone());
        requires_message(_server_command_msg_order.clone());
        requires_message(_server_command_msg_status.clone());
    }
}
