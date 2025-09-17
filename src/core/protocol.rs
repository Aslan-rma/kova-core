//! Protocol management for Kova Core

use crate::core::Error;
use serde::{Deserialize, Serialize};

/// Protocol manager for handling communication protocols
pub struct ProtocolManager {
    protocols: std::collections::HashMap<String, Box<dyn Protocol>>,
}

/// Protocol trait
pub trait Protocol: Send + Sync {
    /// Get protocol name
    fn name(&self) -> &str;
    
    /// Get protocol version
    fn version(&self) -> &str;
    
    /// Initialize protocol
    async fn initialize(&mut self) -> Result<(), Error>;
    
    /// Send message
    async fn send(&self, message: &Message) -> Result<(), Error>;
    
    /// Receive message
    async fn receive(&self) -> Result<Message, Error>;
}

/// Message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub protocol: String,
    pub data: Vec<u8>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ProtocolManager {
    /// Create a new protocol manager
    pub fn new() -> Self {
        Self {
            protocols: std::collections::HashMap::new(),
        }
    }

    /// Add a protocol
    pub fn add_protocol(&mut self, name: String, protocol: Box<dyn Protocol>) {
        self.protocols.insert(name, protocol);
    }

    /// Get a protocol
    pub fn get_protocol(&self, name: &str) -> Option<&Box<dyn Protocol>> {
        self.protocols.get(name)
    }
}
