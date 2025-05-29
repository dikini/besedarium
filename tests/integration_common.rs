//! Integration Tests for Besedarium Session Types Library
//!
//! This module contains integration tests that demonstrate complex protocol scenarios
//! using the modern foundation types and architecture.

use besedarium::{
    impl_traits_for_label, // Import the macro directly
    // Grouped imports from besedarium crate
    BiDirectionalAction,
    ChanId as ChanIdTrait,
    HasDual,
    InputAction,
    Message as MessageTrait, // Renamed to avoid conflict
    OutputAction,
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
impl_traits_for_label!(LLogin);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LAck;
impl_traits_for_label!(LAck);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LQuoteRequest;
impl_traits_for_label!(LQuoteRequest);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LQuoteResponse;
impl_traits_for_label!(LQuoteResponse);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoginLbl;
impl_traits_for_label!(LoginLbl);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AckLbl;
impl_traits_for_label!(AckLbl);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataLbl;
impl_traits_for_label!(DataLbl);

// New Message Labels for Query Protocol
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QueryLbl;
impl_traits_for_label!(QueryLbl);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResultLbl;
impl_traits_for_label!(ResultLbl);

// New Message Labels for Complex Data Types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserProfileLbl;
impl_traits_for_label!(UserProfileLbl);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderDetailsLbl;
impl_traits_for_label!(OrderDetailsLbl);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServerCommandLbl;
impl_traits_for_label!(ServerCommandLbl);

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
impl SessionType for UserProfile {}
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
impl SessionType for OrderDetails {} // Required for HasDual
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
impl SessionType for ServerCommand {} // Required for HasDual
impl HasDual for ServerCommand {
    type Dual = ServerCommand;
}

// --- New Message Types Wrapping Complex Data ---
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserProfileMsg(pub UserProfile);
impl MessageTrait for UserProfileMsg {}
impl SessionType for UserProfileMsg {} // Required for HasDual
impl HasDual for UserProfileMsg {
    type Dual = UserProfileMsg;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderDetailsMsg(pub OrderDetails);
impl MessageTrait for OrderDetailsMsg {}
impl SessionType for OrderDetailsMsg {} // Required for HasDual
impl HasDual for OrderDetailsMsg {
    type Dual = OrderDetailsMsg;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerCommandMsg(pub ServerCommand);
impl MessageTrait for ServerCommandMsg {}
impl SessionType for ServerCommandMsg {} // Required for HasDual
impl HasDual for ServerCommandMsg {
    type Dual = ServerCommandMsg;
}

// --- Message Types (moved from common_type_sanity_tests) ---
// These are basic message types used across integration tests.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginMsg(pub String, pub String);
impl MessageTrait for LoginMsg {}
impl SessionType for LoginMsg {}
impl HasDual for LoginMsg {
    type Dual = LoginMsg;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AckMsg(pub bool, pub Option<String>);
impl MessageTrait for AckMsg {}
impl SessionType for AckMsg {}
impl HasDual for AckMsg {
    type Dual = AckMsg;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataMsg(pub Vec<u8>);
impl MessageTrait for DataMsg {}
impl SessionType for DataMsg {}
impl HasDual for DataMsg {
    type Dual = DataMsg;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryMsg(pub String);
impl MessageTrait for QueryMsg {}
impl SessionType for QueryMsg {}
impl HasDual for QueryMsg {
    type Dual = QueryMsg;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultMsg(pub String);
impl MessageTrait for ResultMsg {}
impl SessionType for ResultMsg {}
impl HasDual for ResultMsg {
    type Dual = ResultMsg;
}


// --- Test Utility Functions (Optional) ---

// Helper function to create a UserProfile for tests
#[cfg(test)]
pub fn create_test_user_profile(id: u64, name: &str) -> UserProfile {
    UserProfile {
        user_id: id,
        username: name.to_string(),
        email: Some(format!("{}@example.com", name)),
        preferences: vec![("theme".to_string(), "dark".to_string())],
        aliases: vec![format!("{}_alias", name)],
    }
}

// Helper function to create OrderDetails for tests
#[cfg(test)]
pub fn create_test_order_details(item_id: &str, quantity: u32) -> OrderDetails {
    OrderDetails {
        item_id: item_id.to_string(),
        quantity,
        notes: Some("Test order".to_string()),
    }
}

// --- Basic Sanity Checks for Common Types ---
// These tests are simple assertions to ensure that the common types
// can be instantiated and meet very basic criteria. They are not exhaustive
// protocol tests but serve as quick checks during development.

#[cfg(test)]
mod common_type_sanity_tests {
    use super::*; // Import everything from the parent module

    #[test]
    fn test_role_instantiation() {
        let _alice = Alice;
        let _bob = Bob;
        let _charlie = Charlie;
        let _buyer = Buyer;
        let _seller1 = Seller1;
        let _seller2 = Seller2;
        let _client = Client;
        let _service = Service;
        let _database = Database;
        // Basic check: ensure types can be created
    }

    #[test]
    fn test_channel_instantiation() {
        let _auth_chan = AuthChan;
        let _order_chan = OrderChan;
        let _query_chan_s1 = QueryChanS1;
        let _query_chan_s2 = QueryChanS2;
        let _order_chan_s1 = OrderChanS1;
        let _order_chan_s2 = OrderChanS2;
        let _data_chan = DataChan;
        let _service_chan = ServiceChan;
        let _db_chan = DbChan;
        // Basic check
    }

    #[test]
    fn test_label_instantiation() {
        let _l_login = LLogin;
        let _l_ack = LAck;
        let _l_quote_req = LQuoteRequest;
        let _l_quote_resp = LQuoteResponse;
        let _login_lbl = LoginLbl;
        let _ack_lbl = AckLbl;
        let _data_lbl = DataLbl;
        let _query_lbl = QueryLbl;
        let _result_lbl = ResultLbl;
        let _user_profile_lbl = UserProfileLbl;
        let _order_details_lbl = OrderDetailsLbl;
        let _server_command_lbl = ServerCommandLbl;
        // Basic check
    }

    #[test]
    fn test_complex_data_creation() {
        let user_profile = create_test_user_profile(1, "testuser");
        assert_eq!(user_profile.user_id, 1);
        assert_eq!(user_profile.username, "testuser");
        assert_eq!(user_profile.email, Some("testuser@example.com".to_string()));

        let order_details = create_test_order_details("item123", 5);
        assert_eq!(order_details.item_id, "item123");
        assert_eq!(order_details.quantity, 5);
        assert_eq!(order_details.notes, Some("Test order".to_string()));

        let server_command_profile = ServerCommand::StoreProfile(user_profile.clone());
        let server_command_order = ServerCommand::ProcessOrder(order_details.clone());
        let server_command_status = ServerCommand::GetStatus;

        // Check that enum variants can be constructed
        match server_command_profile {
            ServerCommand::StoreProfile(up) => assert_eq!(up, user_profile),
            _ => panic!("Unexpected enum variant"),
        }
        match server_command_order {
            ServerCommand::ProcessOrder(od) => assert_eq!(od, order_details),
            _ => panic!("Unexpected enum variant"),
        }
        match server_command_status {
            ServerCommand::GetStatus => {} // Correct variant
            _ => panic!("Unexpected enum variant"),
        }
    }

    #[test]
    fn test_message_wrapper_creation() {
        let user_profile = create_test_user_profile(2, "anotheruser");
        let user_profile_msg = UserProfileMsg(user_profile.clone());
        assert_eq!(user_profile_msg.0, user_profile);

        let order_details = create_test_order_details("item456", 10);
        let order_details_msg = OrderDetailsMsg(order_details.clone());
        assert_eq!(order_details_msg.0, order_details);

        let server_command = ServerCommand::GetStatus;
        let server_command_msg = ServerCommandMsg(server_command.clone());
        assert_eq!(server_command_msg.0, server_command);
    }

    #[test]
    fn test_basic_message_creation() {
        let login_msg = LoginMsg("user".to_string(), "pass".to_string());
        assert_eq!(login_msg.0, "user");
        assert_eq!(login_msg.1, "pass");

        let ack_msg_ok = AckMsg(true, Some("token".to_string()));
        assert!(ack_msg_ok.0);
        assert_eq!(ack_msg_ok.1, Some("token".to_string()));
        
        let ack_msg_fail = AckMsg(false, None);
        assert!(!ack_msg_fail.0);
        assert_eq!(ack_msg_fail.1, None);

        let data_msg = DataMsg(vec![1, 2, 3]);
        assert_eq!(data_msg.0, vec![1, 2, 3]);

        let query_msg = QueryMsg("SELECT * FROM data_table".to_string());
        assert_eq!(query_msg.0, "SELECT * FROM data_table");

        let result_msg = ResultMsg("id: 1, value: example".to_string());
        assert_eq!(result_msg.0, "id: 1, value: example");
    }
}
