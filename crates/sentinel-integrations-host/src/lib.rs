use async_trait::async_trait;
use chrono::Utc;
use sentinel_core::{
    ActionExecutor, ActionResult, Capability, CandidateAction, Collector, Incident, IntegrationAdapter,
    IntegrationStatus, IntegrationType, SentinelError, SignalEvent,
};

pub struct HostAdapter {
    integration_id: String,
}

impl HostAdapter {
    pub fn new(integration_id: impl Into<String>) -> Self {
        Self {
            integration_id: integration_id.into(),
        }
    }
}

#[async_trait]
impl Collector for HostAdapter {
    fn id(&self) -> &str {
        &self.integration_id
    }

    fn integration_type(&self) -> IntegrationType {
        IntegrationType::Host
    }

    async fn collect(&self) -> Result<Vec<SignalEvent>, SentinelError> {
        Ok(Vec::new())
    }
}

#[async_trait]
impl IntegrationAdapter for HostAdapter {
    fn id(&self) -> &str {
        &self.integration_id
    }

    fn integration_type(&self) -> IntegrationType {
        IntegrationType::Host
    }

    fn capabilities(&self) -> Vec<Capability> {
        vec![Capability::ReadSignals, Capability::RestartService]
    }

    async fn health_check(&self) -> Result<IntegrationStatus, SentinelError> {
        Ok(IntegrationStatus {
            integration_id: self.integration_id.clone(),
            integration_type: IntegrationType::Host,
            healthy: true,
            capabilities: self.capabilities(),
            details: "host adapter initialized".to_string(),
        })
    }
}

pub struct HostServiceRestartExecutor;

#[async_trait]
impl ActionExecutor for HostServiceRestartExecutor {
    fn action_type(&self) -> &str {
        "service_restart"
    }

    async fn execute(
        &self,
        _incident: &Incident,
        action: &CandidateAction,
    ) -> Result<ActionResult, SentinelError> {
        Ok(ActionResult {
            action_id: action.action_id.clone(),
            target_id: action.target_id.clone(),
            success: true,
            executed: true,
            details: "stub: service restart dispatched".to_string(),
            finished_at: Utc::now(),
        })
    }
}
