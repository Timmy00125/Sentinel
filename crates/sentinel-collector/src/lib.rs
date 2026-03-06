use sentinel_core::{Collector, SentinelError, SignalEvent};
use std::sync::Arc;

pub struct CompositeCollector {
    collectors: Vec<Arc<dyn Collector>>,
}

impl CompositeCollector {
    pub fn new() -> Self {
        Self {
            collectors: Vec::new(),
        }
    }

    pub fn with_collectors(collectors: Vec<Arc<dyn Collector>>) -> Self {
        Self { collectors }
    }

    pub fn register(&mut self, collector: Arc<dyn Collector>) {
        self.collectors.push(collector);
    }

    pub async fn collect_once(&self) -> Result<Vec<SignalEvent>, SentinelError> {
        let mut all_signals = Vec::new();
        for collector in &self.collectors {
            let mut signals = collector.collect().await?;
            all_signals.append(&mut signals);
        }
        Ok(all_signals)
    }
}

impl Default for CompositeCollector {
    fn default() -> Self {
        Self::new()
    }
}
