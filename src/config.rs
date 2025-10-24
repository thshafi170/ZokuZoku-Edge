use anyhow::Result;
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZokuZokuConfig {
    pub enabled: bool,
    pub game_data_dir: Option<String>,
    pub localize_dict_dump: Option<String>,
    pub auto_download_bundles: bool,
    pub sqlite3: String,
    pub use_game_font: bool,
    pub custom_font: Option<String>,
    pub hachimi_ipc_address: String,
    pub decryption: DecryptionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptionConfig {
    pub enabled: bool,
    pub meta_key: Option<String>,
}

impl Default for ZokuZokuConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            game_data_dir: None,
            localize_dict_dump: None,
            auto_download_bundles: true,
            sqlite3: "sqlite3".to_string(),
            use_game_font: true,
            custom_font: None,
            hachimi_ipc_address: "127.0.0.1".to_string(),
            decryption: DecryptionConfig {
                enabled: true,
                meta_key: None,
            },
        }
    }
}

pub struct ConfigManager {
    config: Config,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let mut config = Config::default();

        // Load defaults
        config.set_default("enabled", false)?;
        config.set_default("auto_download_bundles", true)?;
        config.set_default("sqlite3", "sqlite3")?;
        config.set_default("use_game_font", true)?;
        config.set_default("hachimi_ipc_address", "127.0.0.1")?;
        config.set_default("decryption.enabled", true)?;

        // Load from config file if it exists
        let config_dir = Self::get_config_dir()?;
        let config_file = config_dir.join("config.json");
        if config_file.exists() {
            config.merge(File::from(config_file))?;
        }

        // Load from environment variables
        config.merge(Environment::with_prefix("ZOKUZOKU"))?;

        Ok(Self { config })
    }

    pub fn get_config(&self) -> Result<ZokuZokuConfig> {
        Ok(self.config.clone().try_into()?)
    }

    pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<T> {
        Ok(self.config.get(key)?)
    }

    pub fn set<T: Serialize>(&mut self, key: &str, value: T) -> Result<()> {
        self.config.set(key, value)?;
        Ok(())
    }

    fn get_config_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Failed to get config directory"))?
            .join("zokuzoku");
        std::fs::create_dir_all(&config_dir)?;
        Ok(config_dir)
    }

    pub fn save(&self) -> Result<()> {
        let config_dir = Self::get_config_dir()?;
        let config_file = config_dir.join("config.json");
        let config_json = serde_json::to_string_pretty(&self.get_config()?)?;
        std::fs::write(config_file, config_json)?;
        Ok(())
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new().expect("Failed to create config manager")
    }
}

