use std::fs;

use crate::config::config::{APP_HOME_DIR, CONFIG_FILE, Config};

pub fn init() -> anyhow::Result<()> {
    let home_dir = dirs::home_dir()
        .expect("Cannot find home directory")
        .join(APP_HOME_DIR);

    if !home_dir.exists() {
        println!("Creating home directory at {:?}", home_dir);
        fs::create_dir_all(&home_dir)?;
    }

    let config_path = home_dir.join(CONFIG_FILE);
    if !config_path.exists() {
        println!("Creating default config at {:?}", config_path);
        let config = Config::default();
        let yaml = serde_yaml::to_string(&config)?;
        fs::write(config_path, yaml)?;
    } else {
        println!("Config file already exists at {:?}", config_path);
    }

    Ok(())
}
