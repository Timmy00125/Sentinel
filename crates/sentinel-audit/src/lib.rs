use async_trait::async_trait;
use sentinel_core::{ActionRecord, AuditSink, Incident, SentinelError};

pub struct StdoutAuditSink;

#[async_trait]
impl AuditSink for StdoutAuditSink {
    async fn write_incident(&self, incident: &Incident) -> Result<(), SentinelError> {
        let serialized = serde_json::to_string(incident)
            .map_err(|e| SentinelError::Internal(format!("incident serialization failed: {e}")))?;
        println!("{serialized}");
        Ok(())
    }

    async fn write_action(&self, record: &ActionRecord) -> Result<(), SentinelError> {
        let serialized = serde_json::to_string(record)
            .map_err(|e| SentinelError::Internal(format!("action serialization failed: {e}")))?;
        println!("{serialized}");
        Ok(())
    }
}
