use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HachimiCommand {
    #[serde(rename = "StoryGotoBlock")]
    StoryGotoBlock {
        block_id: u32,
        incremental: bool,
    },
    #[serde(rename = "ReloadLocalizedData")]
    ReloadLocalizedData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HachimiResponse {
    #[serde(rename = "Ok")]
    Ok,
    #[serde(rename = "Error")]
    Error { message: Option<String> },
    #[serde(rename = "HelloWorld")]
    HelloWorld { message: String },
}

pub struct HachimiIpc {
    client: Client,
    address: String,
    port: u16,
}

impl HachimiIpc {
    pub fn new(address: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            address,
            port: 50433,
        }
    }

    pub async fn call(&self, command: HachimiCommand) -> Result<HachimiResponse> {
        let url = format!("http://{}:{}/", self.address, self.port);
        
        let response = self
            .client
            .post(&url)
            .json(&command)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            anyhow::bail!("HTTP error: {}", status);
        }

        let response_data: HachimiResponse = response.json().await?;
        
        match &response_data {
            HachimiResponse::Error { message } => {
                anyhow::bail!(
                    "Hachimi error: {}",
                    message.as_deref().unwrap_or("Unknown error")
                );
            }
            _ => Ok(response_data),
        }
    }

    pub async fn call_with_progress(
        &self,
        command: HachimiCommand,
    ) -> Result<HachimiResponse> {
        // In Zed, we might want to show progress differently
        // For now, just call the normal method
        self.call(command).await
    }
}

impl Default for HachimiIpc {
    fn default() -> Self {
        Self::new("127.0.0.1".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hachimi_command_serialization() {
        let cmd = HachimiCommand::ReloadLocalizedData;
        let json = serde_json::to_string(&cmd).unwrap();
        assert_eq!(json, r#"{"type":"ReloadLocalizedData"}"#);
    }

    #[test]
    fn test_hachimi_command_story_goto_block() {
        let cmd = HachimiCommand::StoryGotoBlock {
            block_id: 42,
            incremental: true,
        };
        let json = serde_json::to_string(&cmd).unwrap();
        println!("{}", json);
    }
}

