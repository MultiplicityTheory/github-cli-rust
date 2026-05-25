pub mod telemetry;
pub mod update;

use anyhow::{Result, anyhow};
use crate::git::GitClient;
use crate::git::gh_repo::GhRepo;

pub fn resolve_repo(repository: Option<String>) -> Result<GhRepo> {
    let git_client = GitClient::new(None); // Current dir
    
    if let Some(repo_name) = repository {
        if repo_name.contains('/') {
            let parts: Vec<&str> = repo_name.split('/').collect();
            Ok(GhRepo::new(parts[0], parts[1], "github.com"))
        } else {
            Err(anyhow!("Repository must be in 'owner/repo' format"))
        }
    } else {
        let remotes = git_client.remotes()?;
        let mut gh_repo = None;
        
        for name in &["upstream", "github", "origin"] {
            if let Some(remote) = remotes.iter().find(|r| r.name == *name) {
                if let Some(parsed) = git_client.parse_gh_repo(&remote.url) {
                    gh_repo = Some(parsed);
                    break;
                }
            }
        }

        if gh_repo.is_none() {
            for remote in remotes {
                if let Some(parsed) = git_client.parse_gh_repo(&remote.url) {
                    gh_repo = Some(parsed);
                    break;
                }
            }
        }

        gh_repo.ok_or_else(|| anyhow!("Could not determine base repository. Try specifying it with 'owner/repo'."))
    }
}
