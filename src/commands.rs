use anyhow::Result;

pub struct CommandHandler {
    // Commands will be registered here
}

impl CommandHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn enable(&self) -> Result<()> {
        // Enable ZokuZoku for the workspace
        tracing::info!("ZokuZoku enabled");
        Ok(())
    }

    pub async fn open_localize_dict_editor(&self) -> Result<()> {
        // Open localize dict editor
        tracing::info!("Opening localize dict editor");
        Ok(())
    }

    pub async fn open_lyrics_editor(&self, song_index: Option<String>) -> Result<()> {
        if let Some(index) = song_index {
            tracing::info!("Opening lyrics editor for song {}", index);
        } else {
            anyhow::bail!("This command cannot be activated manually");
        }
        Ok(())
    }

    pub async fn open_mdb_editor(&self, table_name: Option<String>) -> Result<()> {
        if let Some(name) = table_name {
            tracing::info!("Opening MDB editor for table {}", name);
        } else {
            anyhow::bail!("This command cannot be activated manually");
        }
        Ok(())
    }

    pub async fn open_story_editor(
        &self,
        story_type: Option<String>,
        story_id: Option<String>,
    ) -> Result<()> {
        if let (Some(stype), Some(id)) = (story_type, story_id) {
            tracing::info!("Opening story editor for {} story {}", stype, id);
        } else {
            anyhow::bail!("This command cannot be activated manually");
        }
        Ok(())
    }

    pub async fn reload_localized_data(&self) -> Result<()> {
        tracing::info!("Reloading localized data");
        // Will call Hachimi IPC
        Ok(())
    }

    pub async fn set_localized_data_dir(&self) -> Result<()> {
        tracing::info!("Setting localized data directory");
        Ok(())
    }

    pub async fn revert_localized_data_dir(&self) -> Result<()> {
        tracing::info!("Reverting localized data directory");
        Ok(())
    }

    pub async fn clear_cache(&self) -> Result<()> {
        tracing::info!("Clearing cache");
        Ok(())
    }
}

impl Default for CommandHandler {
    fn default() -> Self {
        Self::new()
    }
}

