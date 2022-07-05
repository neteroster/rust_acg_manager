use std::{path::Path, io::Error, fs::{self, File}};
use serde_json;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct MusicConfig {
    pub issue_id: u64,
    pub enable: bool,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ProgressConfig {
    pub issue_id: u64,
    pub enable: bool,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct AllConfig {
    pub username: String,
    pub repo_name: String,
    pub music_config: MusicConfig,
    pub progress_config: ProgressConfig,
}


impl AllConfig {
    pub fn from_file(dir: &Path) -> Result<AllConfig, Error> {
        let cfg = fs::read_to_string(dir)?;
        let cfg : AllConfig = serde_json::from_str(&cfg)?;
        Ok(cfg)
    }
}
