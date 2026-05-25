use anyhow::{Result, anyhow};
use std::sync::Arc;
use crate::factory::Factory;
use crate::git::GitClient;
use crate::git::gh_repo::GhRepo;
use crate::api::ApiClient;
use serde::Deserialize;

pub async fn view(factory: Arc<Factory>, repository: Option<String>) -> Result<()> {
    let repo = crate::utils::resolve_repo(repository)?;
    let api_client = ApiClient::new(factory.config.clone(), &repo.hostname)?;
    
    #[derive(Deserialize, Debug)]
    struct RepoDetails {
        full_name: String,
        description: Option<String>,
        html_url: String,
        stargazers_count: u32,
        forks_count: u32,
    }

    let details: RepoDetails = api_client.get(&format!("repos/{}", repo.full_name())).await?;

    println!("{}", details.full_name);
    if let Some(desc) = details.description {
        println!("{}", desc);
    }
    println!();
    println!("⭐ {} stars  🍴 {} forks", details.stargazers_count, details.forks_count);
    println!("{}", details.html_url);

    Ok(())
}

pub async fn clone(_factory: Arc<Factory>, repository: String) -> Result<()> {
    // repository could be "owner/repo" or a full URL
    let url = if repository.starts_with("http") || repository.starts_with("git@") {
        repository.clone()
    } else {
        format!("https://github.com/{}.git", repository)
    };

    println!("Cloning into {}...", repository);
    
    let status = std::process::Command::new("git")
        .args(&["clone", &url])
        .status()?;

    if !status.success() {
        return Err(anyhow!("git clone failed"));
    }

    Ok(())
}
