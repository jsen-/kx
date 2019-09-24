mod config;
mod findup;
mod search_path;

use crate::Error;
pub use config::Config;
use search_path::SearchPath;
use std::env;
use std::fs;
use std::io::BufReader;
use std::path::Path;

pub struct Settings {
    pub search_paths: Vec<SearchPath>,
}

fn from_file<P: AsRef<Path>>(path: P) -> Result<Settings, Error> {
    let path = path.as_ref();
    let config_file = fs::File::open(path).map_err(|e| Error::Config(path.into(), e))?;
    let config: Config = serde_json::from_reader(BufReader::new(config_file))
        .map_err(|e| Error::ConfigDeserialize(path.into(), e))?;
    Ok(Settings {
        search_paths: config
            .search_dir
            .into_iter()
            .map(|dir| SearchPath::new(dir, Some(path.to_owned())))
            .collect(),
    })
}

pub fn settings() -> Result<Settings, Error> {
    if let Some(search_dir) = env::var_os("KX_SEARCH_DIR") {
        return Ok(Settings {
            search_paths: vec![SearchPath::new(search_dir.into(), None)],
        });
    }
    if let Some(config_path) = env::var_os("KX_CONFIG_PATH") {
        return from_file(&config_path);
    }
    if let Some(xdg_config_home) = env::var_os("XDG_CONFIG_HOME") {
        let config_file = Path::new(&xdg_config_home).join("kx").join("config.json");
        if config_file.exists() {
            return from_file(&config_file);
        }
    }
    if let Some(home_dir) = dirs::home_dir() {
        let config_file = Path::new(&home_dir)
            .join(".config")
            .join("kx")
            .join("config.json");
        if config_file.exists() {
            return from_file(&config_file);
        }
        Ok(Settings {
            search_paths: vec![SearchPath::new(home_dir.join(".kube"), None)],
        })
    } else {
        Err(Error::HomeDir)?
    }
}
