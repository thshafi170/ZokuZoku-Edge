use anyhow::Result;
use std::path::{Path, PathBuf};

pub fn expand_environment_variables(path: &str) -> String {
    if cfg!(windows) {
        // On Windows, expand %VAR% syntax
        let mut result = path.to_string();
        for (key, value) in std::env::vars() {
            let pattern = format!("%{}%", key);
            result = result.replace(&pattern, &value);
        }
        result
    } else {
        // On Unix, expand $VAR or ${VAR} syntax
        let mut result = String::new();
        let mut chars = path.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '$' {
                if let Some('{') = chars.peek() {
                    chars.next(); // consume '{'
                    let mut var_name = String::new();
                    while let Some(ch) = chars.next() {
                        if ch == '}' {
                            break;
                        }
                        var_name.push(ch);
                    }
                    if let Ok(value) = std::env::var(&var_name) {
                        result.push_str(&value);
                    }
                } else {
                    let mut var_name = String::new();
                    while let Some(ch) = chars.peek() {
                        if ch.is_alphanumeric() || *ch == '_' {
                            var_name.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    if let Ok(value) = std::env::var(&var_name) {
                        result.push_str(&value);
                    }
                }
            } else {
                result.push(ch);
            }
        }
        result
    }
}

pub fn normalize_story_id(story_id: &str) -> String {
    // Ensure story ID is zero-padded
    if story_id.len() < 9 {
        format!("{:0>9}", story_id)
    } else {
        story_id.to_string()
    }
}

pub fn uri_exists(path: &PathBuf) -> bool {
    path.exists()
}

pub fn make_active_status_label(label: &str, dict_exists: bool) -> String {
    if dict_exists {
        format!("✓ {}", label)
    } else {
        format!("○ {}", label)
    }
}

pub fn get_all_game_install_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    
    if cfg!(windows) {
        // Windows: Common locations
        let user_profile = std::env::var("USERPROFILE").unwrap_or_default();
        let potential_paths = vec![
            PathBuf::from(format!("{}\\AppData\\Local\\DMM Games\\umamusume", user_profile)),
            PathBuf::from(format!("{}\\Desktop\\umamusume", user_profile)),
            PathBuf::from("C:\\Program Files\\DMM Games\\umamusume"),
        ];
        
        for path in potential_paths {
            if path.exists() {
                paths.push(path);
            }
        }
    } else {
        // Linux: Common locations
        let home = std::env::var("HOME").unwrap_or_default();
        let potential_paths = vec![
            PathBuf::from(format!("{}/.local/share/dmmgames/umamusume", home)),
            PathBuf::from(format!("{}/umamusume", home)),
        ];
        
        for path in potential_paths {
            if path.exists() {
                paths.push(path);
            }
        }
    }
    
    paths
}

pub fn get_default_game_data_dir() -> Result<PathBuf> {
    if cfg!(windows) {
        let user_profile = std::env::var("USERPROFILE")?;
        Ok(PathBuf::from(format!(
            "{}\\AppData\\LocalLow\\Cygames\\umamusume",
            user_profile
        )))
    } else {
        let home = std::env::var("HOME")?;
        Ok(PathBuf::from(format!(
            "{}/.local/share/umamusume",
            home
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_story_id() {
        assert_eq!(normalize_story_id("123"), "000000123");
        assert_eq!(normalize_story_id("123456789"), "123456789");
    }

    #[test]
    fn test_make_active_status_label() {
        assert_eq!(make_active_status_label("Test", true), "✓ Test");
        assert_eq!(make_active_status_label("Test", false), "○ Test");
    }
}

