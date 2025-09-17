//! Storage management for Kova Core

use crate::core::Error;
use std::path::Path;

/// Storage manager for handling data storage
pub struct StorageManager {
    base_path: String,
}

impl StorageManager {
    /// Create a new storage manager
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }

    /// Store data
    pub async fn store(&self, key: &str, data: &[u8]) -> Result<(), Error> {
        let path = Path::new(&self.base_path).join(key);
        
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        std::fs::write(&path, data)?;
        Ok(())
    }

    /// Retrieve data
    pub async fn retrieve(&self, key: &str) -> Result<Vec<u8>, Error> {
        let path = Path::new(&self.base_path).join(key);
        let data = std::fs::read(&path)?;
        Ok(data)
    }

    /// Delete data
    pub async fn delete(&self, key: &str) -> Result<(), Error> {
        let path = Path::new(&self.base_path).join(key);
        std::fs::remove_file(&path)?;
        Ok(())
    }
}
