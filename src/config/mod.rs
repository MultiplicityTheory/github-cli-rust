use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};
use directories::ProjectDirs;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub hosts: HashMap<String, HostConfig>,
    #[serde(default)]
    pub defaults: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HostConfig {
    pub user: String,
    pub oauth_token: String,
    pub git_protocol: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if !path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file at {:?}", path))?;
        
        if content.trim().is_empty() {
            return Ok(Config::default());
        }

        serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse config YAML at {:?}", path))
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_yaml::to_string(self)?;
        fs::write(&path, content)
            .with_context(|| format!("Failed to write config file at {:?}", path))
    }

    pub fn login(&mut self, hostname: &str, user: &str, token: &str, git_protocol: Option<String>) {
        self.hosts.insert(hostname.to_string(), HostConfig {
            user: user.to_string(),
            oauth_token: token.to_string(),
            git_protocol,
        });
    }

    pub fn token_for_host(&self, hostname: &str) -> Option<String> {
        self.hosts.get(hostname).map(|h| h.oauth_token.clone())
    }

    fn config_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "github", "gh-rs")
            .context("Could not determine config directory")?;
        Ok(proj_dirs.config_dir().join("hosts.yml"))
    }
}
