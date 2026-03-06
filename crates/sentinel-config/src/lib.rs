use sentinel_core::AutonomyMode;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentinelConfig {
    pub mode: AutonomyMode,
    #[serde(default)]
    pub integrations: Vec<IntegrationConfig>,
    #[serde(default)]
    pub detection: DetectionConfig,
    #[serde(default)]
    pub notifications: NotificationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub id: String,
    pub kind: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub settings: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionConfig {
    #[serde(default = "default_interval_seconds")]
    pub interval_seconds: u64,
    #[serde(default)]
    pub rules: Vec<RuleConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfig {
    pub id: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub threshold: Option<f64>,
    #[serde(default)]
    pub window_seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    #[serde(default)]
    pub slack_webhook: Option<String>,
    #[serde(default)]
    pub generic_webhook: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    #[serde(default = "default_max_actions")]
    pub max_actions_per_hour: u64,
    #[serde(default = "default_max_actions")]
    pub max_actions_per_target_per_hour: u64,
    #[serde(default)]
    pub allowed_actions: Vec<String>,
    #[serde(default)]
    pub denied_targets: Vec<String>,
    #[serde(default)]
    pub confidence_thresholds: BTreeMap<String, f32>,
    #[serde(default = "default_cooldown")]
    pub global_cooldown_seconds: u64,
    #[serde(default = "default_downgrade_failures")]
    pub auto_downgrade_after_failures: u32,
}

fn default_interval_seconds() -> u64 {
    30
}

fn default_max_actions() -> u64 {
    4
}

fn default_cooldown() -> u64 {
    600
}

fn default_downgrade_failures() -> u32 {
    3
}

impl Default for DetectionConfig {
    fn default() -> Self {
        Self {
            interval_seconds: default_interval_seconds(),
            rules: Vec::new(),
        }
    }
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            slack_webhook: None,
            generic_webhook: None,
        }
    }
}
