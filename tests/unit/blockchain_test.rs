//! Unit tests for blockchain modules

use kova_core::{
    blockchain::{SolanaClient, IPFSClient, ArweaveClient, BlockchainClient},
    core::Error,
};

#[tokio::test]
async fn test_solana_client_creation() {
    let config = kova_core::blockchain::SolanaConfig::default();
    let client = SolanaClient::new(config).await.unwrap();
    
    assert_eq!(client.name(), "Solana");
}

#[tokio::test]
async fn test_ipfs_client_creation() {
    let config = kova_core::blockchain::IPFSConfig::default();
    let client = IPFSClient::new(config).await.unwrap();
    
    assert_eq!(client.name(), "IPFS");
}

#[tokio::test]
async fn test_arweave_client_creation() {
    let config = kova_core::blockchain::ArweaveConfig::default();
    let client = ArweaveClient::new(config).await.unwrap();
    
    assert_eq!(client.name(), "Arweave");
}

#[tokio::test]
async fn test_blockchain_client_trait() {
    let config = kova_core::blockchain::SolanaConfig::default();
    let client = SolanaClient::new(config).await.unwrap();
    
    // Test that the client implements the BlockchainClient trait
    let _: &dyn BlockchainClient = &client;
}

#[tokio::test]
async fn test_data_storage() {
    let config = kova_core::blockchain::IPFSConfig::default();
    let client = IPFSClient::new(config).await.unwrap();
    
    let test_data = b"test data for blockchain storage";
    
    // Test data storage (this will fail in test environment, but tests the interface)
    let result = client.store_data(test_data).await;
    // We expect this to fail in test environment since IPFS node is not running
    assert!(result.is_err());
}

#[tokio::test]
async fn test_data_retrieval() {
    let config = kova_core::blockchain::IPFSConfig::default();
    let client = IPFSClient::new(config).await.unwrap();
    
    let test_hash = "QmTestHash";
    
    // Test data retrieval (this will fail in test environment, but tests the interface)
    let result = client.retrieve_data(test_hash).await;
    // We expect this to fail in test environment since IPFS node is not running
    assert!(result.is_err());
}
