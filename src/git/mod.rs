pub mod gh_repo;

use std::process::Command;
use anyhow::{Result, anyhow};
use std::path::PathBuf;
use url::Url;
use self::gh_repo::GhRepo;

pub struct GitClient {
    repo_dir: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Remote {
    pub name: String,
    pub url: String,
    pub push: bool,
    pub fetch: bool,
    pub resolved: bool,
}

impl GitClient {
    pub fn new(repo_dir: Option<PathBuf>) -> Self {
        Self { repo_dir }
    }

    pub fn command(&self, args: &[&str]) -> Command {
        let mut cmd = Command::new("git");
        if let Some(ref dir) = self.repo_dir {
            cmd.current_dir(dir);
        }
        cmd.args(args);
        cmd
    }

    pub fn remotes(&self) -> Result<Vec<Remote>> {
        let output = self.command(&["remote", "-v"]).output()?;
        if !output.status.success() {
            return Err(anyhow!("git remote -v failed"));
        }

        let stdout = String::from_utf8(output.stdout)?;
        let mut remotes: Vec<Remote> = Vec::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 {
                continue;
            }

            let name = parts[0].to_string();
            let url = parts[1].to_string();
            let type_str = parts[2];

            let fetch = type_str.contains("(fetch)");
            let push = type_str.contains("(push)");

            if let Some(remote) = remotes.iter_mut().find(|r| r.name == name && r.url == url) {
                if fetch { remote.fetch = true; }
                if push { remote.push = true; }
            } else {
                remotes.push(Remote {
                    name,
                    url,
                    fetch,
                    push,
                    resolved: false,
                });
            }
        }

        Ok(remotes)
    }

    pub fn current_branch(&self) -> Result<String> {
        let output = self.command(&["rev-parse", "--abbrev-ref", "HEAD"]).output()?;
        if !output.status.success() {
            return Err(anyhow!("failed to get current branch"));
        }
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    pub fn parse_gh_repo(&self, url_str: &str) -> Option<GhRepo> {
        // Simple heuristic for common formats
        // https://github.com/owner/repo.git
        // git@github.com:owner/repo.git
        
        let normalized = if url_str.starts_with("git@") {
            format!("ssh://{}", url_str.replace(":", "/"))
        } else {
            url_str.to_string()
        };

        if let Ok(url) = Url::parse(&normalized) {
            let host = url.host_str()?;
            if !host.contains("github.com") {
                return None;
            }

            let path = url.path().trim_start_matches('/').trim_end_matches(".git");
            let parts: Vec<&str> = path.split('/').collect();
            if parts.len() >= 2 {
                return Some(GhRepo::new(parts[0], parts[1], host));
            }
        }

        None
    }
}
