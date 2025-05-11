//! Types for protocol examples and tests: roles, messages, IO markers.

// --- Concrete Roles ---
pub struct TClient;
pub struct TServer;
pub struct TBroker;
pub struct TWorker;

// --- Example Messages ---
pub struct Message;
pub struct Response;
pub struct Publish;
pub struct Notify;
pub struct Subscribe;

// --- IO protocol marker types ---
pub struct Http;
pub struct Db;
pub struct Mqtt;
pub struct Cache;
pub struct Mixed;
