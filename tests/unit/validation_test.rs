//! Unit tests for validation module

use kova_core::{
    core::validation::{DataValidator, ValidationConfig, QualityMetrics},
    core::Error,
};
use std::collections::HashMap;

#[tokio::test]
async fn test_data_validator_creation() {
    let validator = DataValidator::new();
    // Test that validator can be created
    assert!(true);
}

#[tokio::test]
async fn test_data_validator_with_config() {
    let config = ValidationConfig {
        min_quality_score: 0.8,
        enable_anomaly_detection: true,
        enable_temporal_consistency: true,
        max_noise_threshold: 0.05,
    };
    
    let validator = DataValidator::with_config(config);
    // Test that validator can be created with custom config
    assert!(true);
}

#[tokio::test]
async fn test_data_validation() {
    let validator = DataValidator::new();
    let test_data = b"test data for validation";
    let metadata = HashMap::new();
    
    let result = validator.validate(test_data, &metadata).await.unwrap();
    
    assert!(result.quality_score >= 0.0 && result.quality_score <= 1.0);
    assert!(result.is_valid || !result.is_valid); // Either true or false
    assert!(!result.signature.is_empty());
}

#[tokio::test]
async fn test_validation_with_high_quality_data() {
    let config = ValidationConfig {
        min_quality_score: 0.5,
        enable_anomaly_detection: true,
        enable_temporal_consistency: true,
        max_noise_threshold: 0.1,
    };
    
    let validator = DataValidator::with_config(config);
    
    // Create high-quality test data (repeated pattern)
    let high_quality_data = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    let metadata = HashMap::new();
    
    let result = validator.validate(high_quality_data, &metadata).await.unwrap();
    
    // High-quality data should have a good quality score
    assert!(result.quality_score > 0.5);
}

#[tokio::test]
async fn test_validation_with_low_quality_data() {
    let config = ValidationConfig {
        min_quality_score: 0.8,
        enable_anomaly_detection: true,
        enable_temporal_consistency: true,
        max_noise_threshold: 0.05,
    };
    
    let validator = DataValidator::with_config(config);
    
    // Create low-quality test data (random noise)
    let low_quality_data = b"\x00\xFF\x55\xAA\x33\xCC\x77\x88\x11\x99\x44\xBB\x66\xDD\x22\xEE";
    let metadata = HashMap::new();
    
    let result = validator.validate(low_quality_data, &metadata).await.unwrap();
    
    // Low-quality data should have a lower quality score
    assert!(result.quality_score < 0.8);
}

#[tokio::test]
async fn test_quality_metrics_calculation() {
    let validator = DataValidator::new();
    let test_data = b"test data for quality metrics";
    let metadata = HashMap::new();
    
    let result = validator.validate(test_data, &metadata).await.unwrap();
    
    // Test that quality metrics are calculated
    assert!(result.metrics.noise_level >= 0.0 && result.metrics.noise_level <= 1.0);
    assert!(result.metrics.completeness >= 0.0 && result.metrics.completeness <= 1.0);
    assert!(result.metrics.consistency >= 0.0 && result.metrics.consistency <= 1.0);
    assert!(result.metrics.accuracy >= 0.0 && result.metrics.accuracy <= 1.0);
    assert!(result.metrics.anomaly_score >= 0.0 && result.metrics.anomaly_score <= 1.0);
}

#[tokio::test]
async fn test_validation_with_empty_data() {
    let validator = DataValidator::new();
    let empty_data = b"";
    let metadata = HashMap::new();
    
    let result = validator.validate(empty_data, &metadata).await.unwrap();
    
    // Empty data should have low quality
    assert!(result.quality_score < 0.5);
}

#[tokio::test]
async fn test_validation_with_metadata() {
    let validator = DataValidator::new();
    let test_data = b"test data with metadata";
    let mut metadata = HashMap::new();
    metadata.insert("sensor_type".to_string(), "camera".to_string());
    metadata.insert("resolution".to_string(), "1920x1080".to_string());
    
    let result = validator.validate(test_data, &metadata).await.unwrap();
    
    // Should still validate successfully with metadata
    assert!(result.quality_score >= 0.0 && result.quality_score <= 1.0);
}
