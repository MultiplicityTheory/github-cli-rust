use anyhow::{Result, anyhow};
use std::sync::Arc;
use crate::factory::Factory;
use crate::api::ApiClient;
use serde::Deserialize;
use tabled::{Table, Tabled};
use inquire::Select;

#[derive(Deserialize, Tabled, Clone)]
pub struct Issue {
    #[tabled(rename = "ID")]
    pub number: u32,
    pub title: String,
    pub state: String,
}

#[derive(Deserialize, Clone)]
pub struct Label {
    pub name: String,
}

impl std::fmt::Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{} {}", self.number, self.title)
    }
}

pub async fn list(factory: Arc<Factory>, repository: Option<String>) -> Result<()> {
    let repo = crate::utils::resolve_repo(repository)?;
    let api_client = ApiClient::new(factory.config.clone(), &repo.hostname)?;

    let issues: Vec<Issue> = api_client.get(&format!("repos/{}/issues?state=open", repo.full_name())).await?;

    if issues.is_empty() {
        println!("No open issues in {}", repo.full_name());
        return Ok(());
    }

    println!("Showing open issues for {}\n", repo.full_name());
    println!("{}", Table::new(issues));

    Ok(())
}

pub async fn view(factory: Arc<Factory>, repository: Option<String>, issue_number: Option<u32>) -> Result<()> {
    let repo = crate::utils::resolve_repo(repository)?;
    let api_client = ApiClient::new(factory.config.clone(), &repo.hostname)?;

    let number = if let Some(n) = issue_number {
        n
    } else {
        // Interactive selection
        let issues: Vec<Issue> = api_client.get(&format!("repos/{}/issues?state=open", repo.full_name())).await?;
        if issues.is_empty() {
            return Err(anyhow!("No open issues to select from"));
        }
        
        let selection = Select::new("Select an issue to view:", issues).prompt()?;
        selection.number
    };

    #[derive(Deserialize)]
    struct IssueDetail {
        number: u32,
        title: String,
        body: Option<String>,
        html_url: String,
    }

    let issue: IssueDetail = api_client.get(&format!("repos/{}/issues/{}", repo.full_name(), number)).await?;

    println!("#{} {}", issue.number, issue.title);
    println!("{}", issue.html_url);
    println!("\n{}\n", issue.body.unwrap_or_else(|| "No description provided.".to_string()));

    Ok(())
}
