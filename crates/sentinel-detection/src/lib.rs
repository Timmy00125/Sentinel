use sentinel_core::{DetectionRule, Incident, SentinelError, SignalEvent};
use std::sync::Arc;

pub struct RuleEngine {
    rules: Vec<Arc<dyn DetectionRule>>,
}

impl RuleEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn register(&mut self, rule: Arc<dyn DetectionRule>) {
        self.rules.push(rule);
    }

    pub async fn detect(&self, signals: &[SignalEvent]) -> Result<Vec<Incident>, SentinelError> {
        let mut incidents = Vec::new();
        for rule in &self.rules {
            let mut found = rule.evaluate(signals).await?;
            incidents.append(&mut found);
        }
        Ok(incidents)
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}
