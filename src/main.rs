use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail};
use clap::Parser;
use time::{format_description, OffsetDateTime};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = ConfigFile::load().unwrap_or_default();
    let file = args.file(&config);

    exec(&args.command, &file)?;

    Ok(())
}

fn exec(cmd: &ArgsCommand, file: &Path) -> anyhow::Result<()> {
    match cmd {
        ArgsCommand::Init => init_config()?,
        ArgsCommand::Log { log } => log_to_file(log, file)?,
    }
    Ok(())
}

fn init_config() -> anyhow::Result<()> {
    let default = ConfigFile::default();
    let default = toml::to_string_pretty(&default)?;
    let cfg_path = config_file_path()?;
    if cfg_path.exists() {
        bail!("Config file {} already exists", cfg_path.to_string_lossy());
    }
    if let Some(parent) = cfg_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&cfg_path, default)?;
    println!("Wrote default config to {}", cfg_path.to_string_lossy());
    Ok(())
}

fn log_to_file(msg: &str, file: &Path) -> anyhow::Result<()> {
    let mut content = std::fs::read_to_string(file).unwrap_or_default();
    let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]")?;
    let date = OffsetDateTime::now_utc().format(&format)?;
    let fmsg = format!("{date}: {msg}\n");
    content.insert_str(0, &fmsg);
    std::fs::write(file, content)?;
    Ok(())
}

#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Log file to use
    #[clap(short, long)]
    file: Option<PathBuf>,

    #[clap(subcommand)]
    command: ArgsCommand,
}

impl Args {
    pub fn file(&self, cfg: &ConfigFile) -> PathBuf {
        if let Some(f) = &self.file {
            f.clone()
        } else {
            cfg.normalized_path()
        }
    }
}

#[derive(Debug, clap::Subcommand)]
pub enum ArgsCommand {
    /// Create a default config file
    Init,

    /// Log a message to the given file
    Log {
        /// Message to log
        log: String,
    },
}

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

fn config_file_path() -> anyhow::Result<PathBuf> {
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
