use sentinel_core::{CandidateAction, DecisionStrategy, Incident, RiskLevel, SentinelError};
use std::collections::BTreeMap;

pub struct SimpleDecisionEngine {
    catalog: BTreeMap<String, Vec<CandidateAction>>,
}

impl SimpleDecisionEngine {
    pub fn new() -> Self {
        Self {
            catalog: BTreeMap::new(),
        }
    }

    pub fn register_catalog_entry(
        &mut self,
        incident_title: String,
        actions: Vec<CandidateAction>,
    ) {
        self.catalog.insert(incident_title, actions);
    }
}

#[async_trait::async_trait]
impl DecisionStrategy for SimpleDecisionEngine {
    fn id(&self) -> &str {
        "simple-decision-engine"
    }

    async fn rank_actions(
        &self,
        incident: &Incident,
    ) -> Result<Vec<CandidateAction>, SentinelError> {
        if let Some(actions) = self.catalog.get(&incident.title) {
            return Ok(actions.clone());
        }

        let fallback = CandidateAction {
            action_id: format!("{}:fallback-restart", incident.incident_id),
            action_type: "service_restart".to_string(),
            target_id: incident.target_id.clone(),
            confidence: 0.50,
            risk: RiskLevel::Medium,
            rationale: "Fallback remediation when no incident-specific catalog entry exists"
                .to_string(),
            metadata: Default::default(),
        };
        Ok(vec![fallback])
    }
}

impl Default for SimpleDecisionEngine {
    fn default() -> Self {
        Self::new()
    }
}
