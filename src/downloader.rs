use anyhow::Result;
use futures_util::StreamExt;
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tracing::info;

pub struct Downloader {
    client: Client,
}

impl Downloader {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(300))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    pub async fn download_to_file(
        &self,
        url: &str,
        display_name: &str,
        destination: PathBuf,
        show_progress: bool,
    ) -> Result<()> {
        info!("Downloading {} from {}", display_name, url);

        let response = self.client.get(url).send().await?;
        let total_size = response.content_length();
        let mut file = File::create(&destination)?;
        let mut stream = response.bytes_stream();

        let mut downloaded: u64 = 0;

        while let Some(chunk) = stream.try_next().await? {
            file.write_all(&chunk)?;
            downloaded += chunk.len() as u64;

            if show_progress {
                if let Some(total) = total_size {
                    let percent = (downloaded as f64 / total as f64) * 100.0;
                    info!("Downloaded {}% of {}", percent as u32, display_name);
                }
            }
        }

        info!("Finished downloading {}", display_name);
        Ok(())
    }

    pub async fn download_as_string(&self, url: &str) -> Result<String> {
        let response = self.client.get(url).send().await?;
        let text = response.text().await?;
        Ok(text)
    }
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_downloader_creation() {
        let downloader = Downloader::new();
        assert!(downloader.client.timeout().is_some());
    }
}

