use sentinel_core::MetricsSink;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct InMemoryMetricsSink {
    counters: Arc<Mutex<BTreeMap<String, u64>>>,
    gauges: Arc<Mutex<BTreeMap<String, f64>>>,
}

impl InMemoryMetricsSink {
    pub fn snapshot_counters(&self) -> BTreeMap<String, u64> {
        self.counters
            .lock()
            .expect("metrics counter lock poisoned")
            .clone()
    }

    pub fn snapshot_gauges(&self) -> BTreeMap<String, f64> {
        self.gauges
            .lock()
            .expect("metrics gauge lock poisoned")
            .clone()
    }
}

impl MetricsSink for InMemoryMetricsSink {
    fn incr_counter(&self, metric: &str, labels: &[(&str, &str)], value: u64) {
        let key = metric_key(metric, labels);
        let mut counters = self.counters.lock().expect("metrics counter lock poisoned");
        *counters.entry(key).or_insert(0) += value;
    }

    fn set_gauge(&self, metric: &str, labels: &[(&str, &str)], value: f64) {
        let key = metric_key(metric, labels);
        let mut gauges = self.gauges.lock().expect("metrics gauge lock poisoned");
        gauges.insert(key, value);
    }
}

fn metric_key(metric: &str, labels: &[(&str, &str)]) -> String {
    if labels.is_empty() {
        return metric.to_string();
    }

    let rendered = labels
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join(",");
    format!("{metric}{{{rendered}}}")
}
