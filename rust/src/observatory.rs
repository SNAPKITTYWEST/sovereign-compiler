// observatory.rs — SnapKitty Runtime Observatory

use serde::{Deserialize, Serialize};

/// Telemetry event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub event_type: String,
    pub declaration_hash: String,
    pub timestamp: String,
    pub data: serde_json::Value,
}

/// SnapKitty Observatory for runtime telemetry.
pub struct Observatory {
    events: Vec<TelemetryEvent>,
}

impl Observatory {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }

    /// Record a telemetry event.
    pub fn record(&mut self, event_type: &str, declaration_hash: &str, data: serde_json::Value) {
        let event = TelemetryEvent {
            event_type: event_type.to_string(),
            declaration_hash: declaration_hash.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            data,
        };
        self.events.push(event);
    }

    /// Get all events for a declaration.
    pub fn get_events(&self, declaration_hash: &str) -> Vec<&TelemetryEvent> {
        self.events
            .iter()
            .filter(|e| e.declaration_hash == declaration_hash)
            .collect()
    }

    /// Get all events.
    pub fn all_events(&self) -> &[TelemetryEvent] {
        &self.events
    }
}
