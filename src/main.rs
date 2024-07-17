use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::bail;
use args::{Args, ArgsCommand};
use clap::Parser;
use config::{config_file_path, ConfigFile};
use query::Query;
use time::{
    format_description::{self},
    OffsetDateTime,
};

mod args;
mod config;
mod query;

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
        ArgsCommand::Query(query) => query_log(query, file)?,
    }
    Ok(())
}

fn query_log(query: &Query, file: &Path) -> anyhow::Result<()> {
    let file = OpenOptions::new().read(true).open(file)?;
    let mut counter = 0;
    let reader = BufReader::new(file).lines();
    for line in reader {
        let line = line?;
        if let Some(line) = query.query_map(&line) {
            println!("{line}");

            // Check if we have exceeded our limit
            if let Some(limit) = query.limit {
                counter += 1;
                if counter >= limit {
                    break;
                }
            }
        }
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
