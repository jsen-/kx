use std::io;
use std::path::PathBuf;

pub struct Settings {
    search_dir: PathBuf,
}

pub fn settings() -> Result<Settings, io::Error> {
    Ok(Settings {
        search_dir: PathBuf::new(),
    })
}
