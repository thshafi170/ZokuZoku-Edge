use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryData {
    pub title: String,
    pub block_list: Vec<StoryBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryBlock {
    pub name: String,
    pub text: String,
    pub next_block: i32,
    pub difference_flag: i32,
    pub cue_id: i32,
    pub choices: Vec<StoryChoice>,
    pub color_texts: Vec<ColorText>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryChoice {
    pub text: String,
    pub next_block: i32,
    pub difference_flag: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorText {
    pub text: String,
}

pub struct DataManager {
    localized_data_dir: Option<PathBuf>,
}

impl DataManager {
    pub fn new() -> Self {
        Self {
            localized_data_dir: None,
        }
    }

    pub fn set_localized_data_dir(&mut self, dir: PathBuf) {
        self.localized_data_dir = Some(dir);
    }

    pub fn get_path(&self, segments: &[&str]) -> Option<PathBuf> {
        let mut path = self.localized_data_dir.as_ref()?.clone();
        for segment in segments {
            path.push(segment);
        }
        Some(path)
    }

    pub async fn save_json<T: Serialize>(
        &self,
        data: &T,
        segments: &[&str],
    ) -> Result<()> {
        let path = self
            .get_path(segments)
            .ok_or_else(|| anyhow::anyhow!("No localized data directory set"))?;
        
        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(data)?;
        std::fs::write(&path, json)?;
        Ok(())
    }

    pub async fn load_json<T: for<'de> Deserialize<'de>>(
        &self,
        segments: &[&str],
    ) -> Result<T> {
        let path = self
            .get_path(segments)
            .ok_or_else(|| anyhow::anyhow!("No localized data directory set"))?;
        
        let content = std::fs::read_to_string(&path)?;
        let data = serde_json::from_str(&content)?;
        Ok(data)
    }
}

impl Default for DataManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_data_manager() {
        let manager = DataManager::new();
        assert!(manager.localized_data_dir.is_none());
    }

    #[tokio::test]
    async fn test_save_and_load_json() {
        let temp_dir = tempdir().unwrap();
        let mut manager = DataManager::new();
        manager.set_localized_data_dir(temp_dir.path().to_path_buf());

        let test_data = StoryData {
            title: "Test Story".to_string(),
            block_list: vec![StoryBlock {
                name: "Block1".to_string(),
                text: "Hello".to_string(),
                next_block: 1,
                difference_flag: 0,
                cue_id: 100,
                choices: vec![],
                color_texts: vec![],
            }],
        };

        manager
            .save_json(&test_data, &["test.json"])
            .await
            .unwrap();

        let loaded: StoryData = manager.load_json(&["test.json"]).await.unwrap();
        assert_eq!(loaded.title, "Test Story");
    }
}

