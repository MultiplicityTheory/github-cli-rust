use serde::Serialize;
use std::sync::{Arc, RwLock};
use crate::config::Config;
use crate::api::ApiClient;

#[derive(Serialize)]
pub struct TelemetryEvent {
    pub command: String,
    pub version: String,
    pub os: String,
    pub arch: String,
}

pub struct Telemetry {
    config: Arc<RwLock<Config>>,
    version: String,
}

impl Telemetry {
    pub fn new(config: Arc<RwLock<Config>>, version: &str) -> Self {
        Self {
            config,
            version: version.to_string(),
        }
    }

    pub async fn log_command(&self, command: &str) {
        // In a real implementation, we would check if telemetry is enabled in config
        // and then send this to a telemetry endpoint via ApiClient
        
        let _event = TelemetryEvent {
            command: command.to_string(),
            version: self.version.clone(),
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
        };

        // For now, we just print a debug message if GH_DEBUG is set
        if std::env::var("GH_DEBUG").is_ok() {
            eprintln!("[telemetry] command: {}", command);
        }
    }
}
