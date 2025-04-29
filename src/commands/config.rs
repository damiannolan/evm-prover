use crate::config::config::{APP_HOME_DIR, CONFIG_FILE, Config};
use clap::Subcommand;
use std::fs;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// View the current configuration
    View {},

    /// Set a configuration key to a new value
    Set { key: String, value: String },
}

fn get_home_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Cannot find home directory")
        .join(APP_HOME_DIR)
}

pub fn handle_config(cmd: ConfigCommands) -> anyhow::Result<()> {
    let config_path = get_home_dir().join(CONFIG_FILE);
    let config_yaml = fs::read_to_string(&config_path)?;
    let mut config: Config = serde_yaml::from_str(&config_yaml)?;

    match cmd {
        ConfigCommands::View {} => {
            println!("{:#?}", config);
        }
        ConfigCommands::Set { key, value } => {
            match key.as_str() {
                "grpc_address" => config.grpc_address = value,
                _ => {
                    println!("Unknown config key: {}", key);
                    return Ok(());
                }
            }
            let yaml = serde_yaml::to_string(&config)?;
            fs::write(config_path, yaml)?;
            println!("Updated config: {:#?}", config);
        }
    }

    Ok(())
}
