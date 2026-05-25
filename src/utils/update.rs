use serde::Deserialize;
use crate::api::ApiClient;
use std::sync::{Arc, RwLock};
use crate::config::Config;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

pub async fn check_for_update(config: Arc<RwLock<Config>>, current_version: &str) -> Option<String> {
    let api_client = ApiClient::new(config, "github.com").ok()?;
    
    let release: Release = api_client.get("repos/cli/cli/releases/latest").await.ok()?;
    
    let latest_version = release.tag_name.trim_start_matches('v');
    let current = current_version.trim_start_matches('v');

    if latest_version != current {
        return Some(latest_version.to_string());
    }

    None
}
