use async_trait::async_trait;
use chrono::Utc;
use sentinel_core::{
    ActionExecutor, ActionResult, Capability, CandidateAction, Collector, Incident, IntegrationAdapter,
    IntegrationStatus, IntegrationType, SentinelError, SignalEvent,
};

pub struct ContainerAdapter {
    integration_id: String,
}

impl ContainerAdapter {
    pub fn new(integration_id: impl Into<String>) -> Self {
        Self {
            integration_id: integration_id.into(),
        }
    }
}

#[async_trait]
impl Collector for ContainerAdapter {
    fn id(&self) -> &str {
        &self.integration_id
    }

    fn integration_type(&self) -> IntegrationType {
        IntegrationType::Container
    }

    async fn collect(&self) -> Result<Vec<SignalEvent>, SentinelError> {
        Ok(Vec::new())
    }
}

#[async_trait]
impl IntegrationAdapter for ContainerAdapter {
    fn id(&self) -> &str {
        &self.integration_id
    }

    fn integration_type(&self) -> IntegrationType {
        IntegrationType::Container
    }

    fn capabilities(&self) -> Vec<Capability> {
        vec![Capability::ReadSignals, Capability::RestartContainer]
    }

    async fn health_check(&self) -> Result<IntegrationStatus, SentinelError> {
        Ok(IntegrationStatus {
            integration_id: self.integration_id.clone(),
            integration_type: IntegrationType::Container,
            healthy: true,
            capabilities: self.capabilities(),
            details: "container adapter initialized".to_string(),
        })
    }
}

pub struct ContainerRestartExecutor;

#[async_trait]
impl ActionExecutor for ContainerRestartExecutor {
    fn action_type(&self) -> &str {
        "container_restart"
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
            details: "stub: container restart dispatched".to_string(),
            finished_at: Utc::now(),
        })
    }
}
