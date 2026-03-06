use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

pub type Metadata = BTreeMap<String, String>;

#[derive(Debug, Error)]
pub enum SentinelError {
    #[error("configuration error: {0}")]
    Configuration(String),
    #[error("policy denied: {0}")]
    PolicyDenied(String),
    #[error("integration failure: {0}")]
    Integration(String),
    #[error("execution failure: {0}")]
    Execution(String),
    #[error("internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AutonomyMode {
    Observe,
    Suggest,
    Apply,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IntegrationType {
    Host,
    Container,
    Kubernetes,
    LoadBalancer,
    Notification,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Capability {
    ReadSignals,
    RestartService,
    RestartContainer,
    RollingRestart,
    BoundedScale,
    WeightAdjust,
    QuarantineTarget,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalEvent {
    pub integration_id: String,
    pub target_id: String,
    pub signal_type: String,
    pub observed_at: DateTime<Utc>,
    pub value: f64,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incident {
    pub incident_id: String,
    pub title: String,
    pub summary: String,
    pub integration_id: String,
    pub target_id: String,
    pub trigger_evidence: Vec<String>,
    pub impact_estimate: String,
    pub created_at: DateTime<Utc>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateAction {
    pub action_id: String,
    pub action_type: String,
    pub target_id: String,
    pub confidence: f32,
    pub risk: RiskLevel,
    pub rationale: String,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDecision {
    pub allowed: bool,
    pub reason: String,
    pub cooldown_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub action_id: String,
    pub target_id: String,
    pub success: bool,
    pub executed: bool,
    pub details: String,
    pub finished_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationStatus {
    pub integration_id: String,
    pub integration_type: IntegrationType,
    pub healthy: bool,
    pub capabilities: Vec<Capability>,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRecord {
    pub incident_id: String,
    pub mode: AutonomyMode,
    pub action: CandidateAction,
    pub policy_decision: PolicyDecision,
    pub result: ActionResult,
}

#[async_trait]
pub trait Collector: Send + Sync {
    fn id(&self) -> &str;
    fn integration_type(&self) -> IntegrationType;
    async fn collect(&self) -> Result<Vec<SignalEvent>, SentinelError>;
}

#[async_trait]
pub trait DetectionRule: Send + Sync {
    fn id(&self) -> &str;
    async fn evaluate(&self, signals: &[SignalEvent]) -> Result<Vec<Incident>, SentinelError>;
}

#[async_trait]
pub trait DecisionStrategy: Send + Sync {
    fn id(&self) -> &str;
    async fn rank_actions(
        &self,
        incident: &Incident,
    ) -> Result<Vec<CandidateAction>, SentinelError>;
}

#[async_trait]
pub trait PolicyEngine: Send + Sync {
    async fn evaluate(
        &self,
        mode: AutonomyMode,
        incident: &Incident,
        action: &CandidateAction,
    ) -> Result<PolicyDecision, SentinelError>;
}

#[async_trait]
pub trait ActionExecutor: Send + Sync {
    fn action_type(&self) -> &str;
    async fn execute(
        &self,
        incident: &Incident,
        action: &CandidateAction,
    ) -> Result<ActionResult, SentinelError>;
}

#[async_trait]
pub trait IntegrationAdapter: Send + Sync {
    fn id(&self) -> &str;
    fn integration_type(&self) -> IntegrationType;
    fn capabilities(&self) -> Vec<Capability>;
    async fn health_check(&self) -> Result<IntegrationStatus, SentinelError>;
}

#[async_trait]
pub trait AuditSink: Send + Sync {
    async fn write_incident(&self, incident: &Incident) -> Result<(), SentinelError>;
    async fn write_action(&self, record: &ActionRecord) -> Result<(), SentinelError>;
}

pub trait MetricsSink: Send + Sync {
    fn incr_counter(&self, metric: &str, labels: &[(&str, &str)], value: u64);
    fn set_gauge(&self, metric: &str, labels: &[(&str, &str)], value: f64);
}
