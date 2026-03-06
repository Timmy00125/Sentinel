use async_trait::async_trait;
use chrono::Utc;
use sentinel_core::{
    ActionExecutor, ActionResult, Capability, CandidateAction, Incident, IntegrationAdapter,
    IntegrationStatus, IntegrationType, SentinelError,
};

pub struct LoadBalancerAdapter {
    integration_id: String,
}

impl LoadBalancerAdapter {
    pub fn new(integration_id: impl Into<String>) -> Self {
        Self {
            integration_id: integration_id.into(),
        }
    }
}

#[async_trait]
impl IntegrationAdapter for LoadBalancerAdapter {
    fn id(&self) -> &str {
        &self.integration_id
    }

    fn integration_type(&self) -> IntegrationType {
        IntegrationType::LoadBalancer
    }

    fn capabilities(&self) -> Vec<Capability> {
        vec![Capability::ReadSignals, Capability::WeightAdjust]
    }

    async fn health_check(&self) -> Result<IntegrationStatus, SentinelError> {
        Ok(IntegrationStatus {
            integration_id: self.integration_id.clone(),
            integration_type: IntegrationType::LoadBalancer,
            healthy: true,
            capabilities: self.capabilities(),
            details: "load balancer adapter initialized".to_string(),
        })
    }
}

pub struct LoadBalancerWeightAdjustExecutor;

#[async_trait]
impl ActionExecutor for LoadBalancerWeightAdjustExecutor {
    fn action_type(&self) -> &str {
        "lb_weight_adjust"
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
            details: "stub: load balancer weight adjustment dispatched".to_string(),
            finished_at: Utc::now(),
        })
    }
}
