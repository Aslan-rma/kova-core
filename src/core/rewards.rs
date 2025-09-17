//! Reward management for Kova Core

use crate::core::Error;
use serde::{Deserialize, Serialize};

/// Reward manager for handling rewards and incentives
pub struct RewardManager {
    rewards: std::collections::HashMap<String, Reward>,
}

/// Reward structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    pub id: String,
    pub validator_id: String,
    pub amount: f64,
    pub currency: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub status: RewardStatus,
}

/// Reward status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RewardStatus {
    Pending,
    Confirmed,
    Failed,
}

impl RewardManager {
    /// Create a new reward manager
    pub fn new() -> Self {
        Self {
            rewards: std::collections::HashMap::new(),
        }
    }

    /// Add a reward
    pub fn add_reward(&mut self, reward: Reward) {
        self.rewards.insert(reward.id.clone(), reward);
    }

    /// Get rewards for a validator
    pub fn get_rewards_for_validator(&self, validator_id: &str) -> Vec<&Reward> {
        self.rewards.values()
            .filter(|reward| reward.validator_id == validator_id)
            .collect()
    }
}
