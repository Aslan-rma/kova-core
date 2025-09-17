//! Data validation and quality assessment

use crate::core::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Data validator for sensor data
pub struct DataValidator {
    config: ValidationConfig,
}

/// Validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Minimum quality score threshold
    pub min_quality_score: f64,
    /// Enable anomaly detection
    pub enable_anomaly_detection: bool,
    /// Enable temporal consistency checks
    pub enable_temporal_consistency: bool,
    /// Maximum noise threshold
    pub max_noise_threshold: f64,
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Overall quality score (0.0 to 1.0)
    pub quality_score: f64,
    /// Validation timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Quality metrics
    pub metrics: QualityMetrics,
    /// Validation signature
    pub signature: String,
    /// Is valid
    pub is_valid: bool,
}

/// Quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Noise level
    pub noise_level: f64,
    /// Completeness score
    pub completeness: f64,
    /// Consistency score
    pub consistency: f64,
    /// Accuracy score
    pub accuracy: f64,
    /// Anomaly score
    pub anomaly_score: f64,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            min_quality_score: 0.7,
            enable_anomaly_detection: true,
            enable_temporal_consistency: true,
            max_noise_threshold: 0.1,
        }
    }
}

impl DataValidator {
    /// Create a new data validator
    pub fn new() -> Self {
        Self {
            config: ValidationConfig::default(),
        }
    }

    /// Create a new data validator with configuration
    pub fn with_config(config: ValidationConfig) -> Self {
        Self { config }
    }

    /// Validate sensor data
    pub async fn validate(&self, data: &[u8], metadata: &HashMap<String, String>) -> Result<ValidationResult, Error> {
        let timestamp = chrono::Utc::now();
        
        // Calculate quality metrics
        let metrics = self.calculate_quality_metrics(data, metadata).await?;
        
        // Calculate overall quality score
        let quality_score = self.calculate_quality_score(&metrics);
        
        // Check if data is valid
        let is_valid = quality_score >= self.config.min_quality_score;
        
        // Generate signature (simplified)
        let signature = self.generate_signature(data, &timestamp);
        
        Ok(ValidationResult {
            quality_score,
            timestamp,
            metrics,
            signature,
            is_valid,
        })
    }

    /// Validate with custom configuration
    pub async fn validate_with_config(
        &self,
        data: &[u8],
        config: &ValidationConfig,
    ) -> Result<ValidationResult, Error> {
        let timestamp = chrono::Utc::now();
        
        // Calculate quality metrics
        let metrics = self.calculate_quality_metrics(data, &HashMap::new()).await?;
        
        // Calculate overall quality score
        let quality_score = self.calculate_quality_score(&metrics);
        
        // Check if data is valid
        let is_valid = quality_score >= config.min_quality_score;
        
        // Generate signature (simplified)
        let signature = self.generate_signature(data, &timestamp);
        
        Ok(ValidationResult {
            quality_score,
            timestamp,
            metrics,
            signature,
            is_valid,
        })
    }

    /// Calculate quality metrics
    async fn calculate_quality_metrics(
        &self,
        data: &[u8],
        _metadata: &HashMap<String, String>,
    ) -> Result<QualityMetrics, Error> {
        // Simplified quality metrics calculation
        let noise_level = self.calculate_noise_level(data);
        let completeness = self.calculate_completeness(data);
        let consistency = self.calculate_consistency(data);
        let accuracy = self.calculate_accuracy(data);
        let anomaly_score = if self.config.enable_anomaly_detection {
            self.calculate_anomaly_score(data).await?
        } else {
            0.0
        };

        Ok(QualityMetrics {
            noise_level,
            completeness,
            consistency,
            accuracy,
            anomaly_score,
        })
    }

    /// Calculate noise level
    fn calculate_noise_level(&self, data: &[u8]) -> f64 {
        // Simplified noise calculation
        if data.is_empty() {
            return 1.0;
        }
        
        let variance = data.iter()
            .map(|&x| (x as f64 - 127.5).powi(2))
            .sum::<f64>() / data.len() as f64;
        
        (variance / 127.5_f64.powi(2)).min(1.0)
    }

    /// Calculate completeness
    fn calculate_completeness(&self, data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        // Check for null bytes or missing data
        let null_count = data.iter().filter(|&&x| x == 0).count();
        let completeness = 1.0 - (null_count as f64 / data.len() as f64);
        completeness.max(0.0)
    }

    /// Calculate consistency
    fn calculate_consistency(&self, data: &[u8]) -> f64 {
        if data.len() < 2 {
            return 1.0;
        }
        
        // Calculate variance in data
        let mean = data.iter().map(|&x| x as f64).sum::<f64>() / data.len() as f64;
        let variance = data.iter()
            .map(|&x| (x as f64 - mean).powi(2))
            .sum::<f64>() / data.len() as f64;
        
        // Lower variance = higher consistency
        (1.0 - (variance / 127.5_f64.powi(2))).max(0.0)
    }

    /// Calculate accuracy
    fn calculate_accuracy(&self, data: &[u8]) -> f64 {
        // Simplified accuracy calculation
        if data.is_empty() {
            return 0.0;
        }
        
        // Check for reasonable data ranges
        let valid_count = data.iter()
            .filter(|&&x| x >= 32 && x <= 126) // Printable ASCII range
            .count();
        
        valid_count as f64 / data.len() as f64
    }

    /// Calculate anomaly score
    async fn calculate_anomaly_score(&self, data: &[u8]) -> Result<f64, Error> {
        // Simplified anomaly detection
        if data.len() < 10 {
            return Ok(0.0);
        }
        
        // Calculate statistical measures
        let mean = data.iter().map(|&x| x as f64).sum::<f64>() / data.len() as f64;
        let std_dev = (data.iter()
            .map(|&x| (x as f64 - mean).powi(2))
            .sum::<f64>() / data.len() as f64).sqrt();
        
        // Count outliers (simplified)
        let outliers = data.iter()
            .filter(|&&x| (x as f64 - mean).abs() > 2.0 * std_dev)
            .count();
        
        Ok(outliers as f64 / data.len() as f64)
    }

    /// Calculate overall quality score
    fn calculate_quality_score(&self, metrics: &QualityMetrics) -> f64 {
        let weights = [0.2, 0.2, 0.2, 0.2, 0.2]; // Equal weights for now
        let scores = [
            1.0 - metrics.noise_level,
            metrics.completeness,
            metrics.consistency,
            metrics.accuracy,
            1.0 - metrics.anomaly_score,
        ];
        
        scores.iter().zip(weights.iter())
            .map(|(score, weight)| score * weight)
            .sum()
    }

    /// Generate validation signature
    fn generate_signature(&self, data: &[u8], timestamp: &chrono::DateTime<chrono::Utc>) -> String {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.update(timestamp.to_rfc3339().as_bytes());
        let hash = hasher.finalize();
        
        hex::encode(hash)
    }
}
