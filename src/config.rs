use anyhow::anyhow;
use std::path::PathBuf;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ConfigFile {
    path: PathBuf,
}

impl ConfigFile {
    pub fn load() -> anyhow::Result<ConfigFile> {
        let path = config_file_path()?;
        let content = std::fs::read_to_string(path)?;
        let config: ConfigFile = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn normalized_path(&self) -> PathBuf {
        shellexpand::tilde(&self.path.to_string_lossy())
            .to_string()
            .into()
    }
}

impl Default for ConfigFile {
    fn default() -> Self {
        ConfigFile {
            path: "log.slf".into(),
        }
    }
}

pub fn config_file_path() -> anyhow::Result<PathBuf> {
    let folder = if cfg!(windows) {
        dirs::config_dir()
            .ok_or_else(|| anyhow!("Config folder missing"))?
            .join("slf/slf.toml")
    } else {
        dirs::home_dir()
            .ok_or_else(|| anyhow!("Home folder missing"))?
            .join(".config/slf/slf.toml")
    };
    Ok(folder)
}
