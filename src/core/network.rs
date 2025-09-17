//! Network management for Kova Core

use crate::core::Error;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Network manager for handling network operations
pub struct NetworkManager {
    connections: RwLock<HashMap<String, Connection>>,
    max_connections: usize,
}

/// Network connection
#[derive(Debug, Clone)]
pub struct Connection {
    pub id: String,
    pub endpoint: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl NetworkManager {
    /// Create a new network manager
    pub fn new(max_connections: usize) -> Self {
        Self {
            connections: RwLock::new(HashMap::new()),
            max_connections,
        }
    }

    /// Add a connection
    pub async fn add_connection(&self, id: String, endpoint: String) -> Result<(), Error> {
        let mut connections = self.connections.write().await;
        
        if connections.len() >= self.max_connections {
            return Err(Error::network("Maximum connections reached"));
        }
        
        let connection = Connection {
            id: id.clone(),
            endpoint,
            is_active: true,
            created_at: chrono::Utc::now(),
        };
        
        connections.insert(id, connection);
        Ok(())
    }

    /// Remove a connection
    pub async fn remove_connection(&self, id: &str) -> Result<(), Error> {
        let mut connections = self.connections.write().await;
        connections.remove(id);
        Ok(())
    }

    /// Get active connections
    pub async fn get_active_connections(&self) -> Vec<Connection> {
        let connections = self.connections.read().await;
        connections.values()
            .filter(|conn| conn.is_active)
            .cloned()
            .collect()
    }
}
