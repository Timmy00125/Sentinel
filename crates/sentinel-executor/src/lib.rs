use chrono::Utc;
use sentinel_core::{
    ActionExecutor, ActionResult, AutonomyMode, CandidateAction, Incident, PolicyEngine,
    SentinelError,
};
use std::collections::BTreeMap;
use std::sync::Arc;

pub struct RemediationOrchestrator {
    executors: BTreeMap<String, Arc<dyn ActionExecutor>>,
}

impl RemediationOrchestrator {
    pub fn new() -> Self {
        Self {
            executors: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, executor: Arc<dyn ActionExecutor>) {
        self.executors
            .insert(executor.action_type().to_string(), executor);
    }

    pub async fn execute_if_permitted(
        &self,
        mode: AutonomyMode,
        policy_engine: &dyn PolicyEngine,
        incident: &Incident,
        action: &CandidateAction,
    ) -> Result<ActionResult, SentinelError> {
        let policy = policy_engine.evaluate(mode, incident, action).await?;
        if !policy.allowed {
            return Ok(ActionResult {
                action_id: action.action_id.clone(),
                target_id: action.target_id.clone(),
                success: false,
                executed: false,
                details: format!("blocked by policy: {}", policy.reason),
                finished_at: Utc::now(),
            });
        }

        if mode != AutonomyMode::Apply {
            return Ok(ActionResult {
                action_id: action.action_id.clone(),
                target_id: action.target_id.clone(),
                success: true,
                executed: false,
                details: "suggest mode: action approved but not executed".to_string(),
                finished_at: Utc::now(),
            });
        }

        let Some(executor) = self.executors.get(&action.action_type) else {
            return Err(SentinelError::Execution(format!(
                "no executor registered for action type '{}'",
                action.action_type
            )));
        };

        executor.execute(incident, action).await
    }
}

impl Default for RemediationOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}
