use anyhow::{Result};
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, DeviceAuthorizationUrl, Scope, TokenResponse, TokenUrl, StandardDeviceAuthorizationResponse};
use std::sync::{Arc};
use crate::factory::Factory;
use crate::api::ApiClient;
use serde::Deserialize;

pub async fn login(factory: Arc<Factory>) -> Result<()> {
    let hostname = "github.com";
    let client_id = ClientId::new("178c6fc778ccc68e1d6a".to_string());
    let client_secret = ClientSecret::new("34ddeff2b558a23d38fba8a6de74f086ede1cc0b".to_string());
    
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())?;
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())?;
    let device_auth_url = DeviceAuthorizationUrl::new("https://github.com/login/device/code".to_string())?;

    let client = BasicClient::new(
        client_id,
        Some(client_secret),
        auth_url,
        Some(token_url),
    )
    .set_device_authorization_url(device_auth_url);

    let details: StandardDeviceAuthorizationResponse = client
        .exchange_device_code()?
        .add_scope(Scope::new("repo".to_string()))
        .add_scope(Scope::new("read:org".to_string()))
        .add_scope(Scope::new("gist".to_string()))
        .request_async(oauth2::reqwest::async_http_client)
        .await?;

    println!("Please visit: {}", details.verification_uri().to_string());
    println!("And enter code: {}", details.user_code().secret());

    let token_result = client
        .exchange_device_access_token(&details)
        .request_async(oauth2::reqwest::async_http_client, tokio::time::sleep, None)
        .await?;

    let token = token_result.access_token().secret();

    // Get current user
    let api_client = ApiClient::new(factory.config.clone(), hostname)?;
    
    #[derive(Deserialize)]
    struct User {
        login: String,
    }

    // Temporary: set token in config so ApiClient can use it
    {
        let mut config = factory.config.write().unwrap();
        config.login(hostname, "pending", token, Some("https".to_string()));
    }

    let user: User = api_client.get("user").await?;
    
    // Update config with real username
    {
        let mut config = factory.config.write().unwrap();
        config.login(hostname, &user.login, token, Some("https".to_string()));
        config.save()?;
    }

    println!("Successfully logged in as {}", user.login);

    Ok(())
}

pub async fn status(factory: Arc<Factory>) -> Result<()> {
    let config = factory.config.read().unwrap();
    if config.hosts.is_empty() {
        println!("You are not logged in to any GitHub hosts. Run 'gh-rs auth login' to get started.");
        return Ok(());
    }

    for (hostname, host_config) in &config.hosts {
        println!("{}:", hostname);
        println!("  Logged in as: {}", host_config.user);
        println!("  Git protocol: {}", host_config.git_protocol.as_deref().unwrap_or("not set"));
        println!("  Token: ******************");
    }

    Ok(())
}
