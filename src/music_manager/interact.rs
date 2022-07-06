use std::{io::Error, str::FromStr};

pub use crate::config::parser::AllConfig;

use super::scanner::{Music, Album, AlbumSet, AudioQuality, CheckSum};
use blake3::Hash;
use serde::{Serialize, Deserialize, de::Visitor};


pub fn generate_line_album(al: &Album, ind_level: usize) -> String {
    let mut res_str = String::new();
    res_str.push_str("    ".repeat(ind_level).as_str());
    res_str.push_str("- ");
    let no_str = String::from("N");
    res_str.push_str(format!("[{}][{}]{}[checksum | {} | {}]"
    ,match &al.id {Some(s) => s, None => &no_str}
    ,al.quality.as_str()
    ,al.title
    ,"BLAKE3"
    ,al.checksum.as_str()
    ).as_str());

    res_str

}

impl Serialize for CheckSum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let cksum = match self {
            CheckSum::Blake3(cksum) => cksum,
        };
        serializer.serialize_str(cksum.to_string().as_str())
    }
}

struct CheckSumVisitor;
impl<'de> Visitor<'de> for CheckSumVisitor {
    type Value = CheckSum;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string of blake3 hash.")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where E: serde::de::Error, {
        let b3hash = Hash::from_str(v);
        match b3hash {
            Ok(b3hash) => Ok(CheckSum::Blake3(b3hash)),
            Err(err) => Err(E::custom("Deserialize hash error."))
        }

    }
}

impl<'de> Deserialize<'de> for CheckSum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        deserializer.deserialize_string(CheckSumVisitor)
    }
}

impl Music {
    pub fn to_markdown(&self) -> String {
        let mut res_str = String::new();
        for album in &self.single_album {
            res_str.push_str(generate_line_album(&album, 0).as_str());
            res_str.push('\n');
        }
        res_str.push('\n');
        for album_set in &self.album_set {
            res_str.push_str(format!("- [Album Set] {}", album_set.title).as_str());
            res_str.push('\n');
            for album in &album_set.albums {
                res_str.push_str(generate_line_album(album, 1).as_str());
                res_str.push('\n');
            }
            res_str.push('\n');
        }

        res_str
    }
    pub async fn push_to_github(&self, cfg: &AllConfig) -> Result<(), octocrab::Error> {
        let ghc_builder = octocrab::OctocrabBuilder::new();
        let markdown_text = self.to_markdown();
        let ghc = ghc_builder
        .personal_token(cfg.access_key.clone())
        .build()?;
        ghc.issues(&cfg.username, &cfg.repo_name)
        .update(cfg.music_config.issue_id)
        .body(markdown_text.as_str())
        .send()
        .await?;
        Ok(())
    }
    pub fn serialize_to_json(&self) -> Result<String, serde_json::Error> {
        match serde_json::to_string(&self) {
            Ok(serialized) => Ok(serialized),
            Err(err) => Err(err),
        }
    }
    pub fn from_json(json_str: &str) -> Result<Music, serde_json::Error> {
        let music : Music = serde_json::from_str(json_str)?;
        Ok(music)
    }
    
}

