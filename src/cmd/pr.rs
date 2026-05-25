use anyhow::{Result, anyhow};
use std::sync::Arc;
use crate::factory::Factory;
use crate::api::ApiClient;
use serde::Deserialize;
use tabled::{Table, Tabled};
use inquire::Select;

#[derive(Deserialize, Tabled, Clone)]
pub struct PullRequest {
    #[tabled(rename = "ID")]
    pub number: u32,
    pub title: String,
    pub state: String,
}

#[derive(Deserialize, Clone)]
pub struct Head {
    #[serde(rename = "ref")]
    pub ref_name: String,
}

impl std::fmt::Display for PullRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{} {}", self.number, self.title)
    }
}

pub async fn list(factory: Arc<Factory>, repository: Option<String>) -> Result<()> {
    let repo = crate::utils::resolve_repo(repository)?;
    let api_client = ApiClient::new(factory.config.clone(), &repo.hostname)?;

    let prs: Vec<PullRequest> = api_client.get(&format!("repos/{}/pulls?state=open", repo.full_name())).await?;

    if prs.is_empty() {
        println!("No open pull requests in {}", repo.full_name());
        return Ok(());
    }

    println!("Showing open pull requests for {}\n", repo.full_name());
    println!("{}", Table::new(prs));

    Ok(())
}

pub async fn view(factory: Arc<Factory>, repository: Option<String>, pr_number: Option<u32>) -> Result<()> {
    let repo = crate::utils::resolve_repo(repository)?;
    let api_client = ApiClient::new(factory.config.clone(), &repo.hostname)?;

    let number = if let Some(n) = pr_number {
        n
    } else {
        // Interactive selection
        let prs: Vec<PullRequest> = api_client.get(&format!("repos/{}/pulls?state=open", repo.full_name())).await?;
        if prs.is_empty() {
            return Err(anyhow!("No open pull requests to select from"));
        }
        
        let selection = Select::new("Select a pull request to view:", prs).prompt()?;
        selection.number
    };

    #[derive(Deserialize)]
    struct PrDetail {
        number: u32,
        title: String,
        body: Option<String>,
        html_url: String,
    }

    let pr: PrDetail = api_client.get(&format!("repos/{}/pulls/{}", repo.full_name(), number)).await?;

    println!("#{} {}", pr.number, pr.title);
    println!("{}", pr.html_url);
    println!("\n{}\n", pr.body.unwrap_or_else(|| "No description provided.".to_string()));

    Ok(())
}
