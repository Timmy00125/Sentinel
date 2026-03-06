use async_trait::async_trait;
use sentinel_config::PolicyConfig;
use sentinel_core::{
    AutonomyMode, CandidateAction, Incident, PolicyDecision, PolicyEngine, SentinelError,
};

pub struct StaticPolicyEngine {
    config: PolicyConfig,
}

impl StaticPolicyEngine {
    pub fn new(config: PolicyConfig) -> Self {
        Self { config }
    }

    fn target_denied(&self, target_id: &str) -> bool {
        self.config.denied_targets.iter().any(|pattern| {
            if let Some(prefix) = pattern.strip_suffix("/*") {
                target_id.starts_with(prefix)
            } else {
                target_id == pattern
            }
        })
    }
}

#[async_trait]
impl PolicyEngine for StaticPolicyEngine {
    async fn evaluate(
        &self,
        mode: AutonomyMode,
        _incident: &Incident,
        action: &CandidateAction,
    ) -> Result<PolicyDecision, SentinelError> {
        if mode == AutonomyMode::Observe {
            return Ok(PolicyDecision {
                allowed: false,
                reason: "observe mode never executes actions".to_string(),
                cooldown_seconds: self.config.global_cooldown_seconds,
            });
        }

        if !self
            .config
            .allowed_actions
            .iter()
            .any(|a| a == &action.action_type)
        {
            return Ok(PolicyDecision {
                allowed: false,
                reason: format!("action '{}' is not allowlisted", action.action_type),
                cooldown_seconds: self.config.global_cooldown_seconds,
            });
        }

        if self.target_denied(&action.target_id) {
            return Ok(PolicyDecision {
                allowed: false,
                reason: format!("target '{}' is denied by policy", action.target_id),
                cooldown_seconds: self.config.global_cooldown_seconds,
            });
        }

        let threshold = self
            .config
            .confidence_thresholds
            .get(&action.action_type)
            .copied()
            .unwrap_or(1.0);
        if action.confidence < threshold {
            return Ok(PolicyDecision {
                allowed: false,
                reason: format!(
                    "confidence {:.2} is below threshold {:.2} for {}",
                    action.confidence, threshold, action.action_type
                ),
                cooldown_seconds: self.config.global_cooldown_seconds,
            });
        }

        Ok(PolicyDecision {
            allowed: true,
            reason: "allowed by policy".to_string(),
            cooldown_seconds: self.config.global_cooldown_seconds,
        })
    }
}
