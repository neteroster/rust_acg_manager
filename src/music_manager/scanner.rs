use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::str::FromStr;
use std::{path::Path, io::Error, io::ErrorKind};
use blake3::{self, Hash};
use walkdir::{self, WalkDir};
use serde::{Serialize, Deserialize};

#[derive(PartialEq)]
#[derive(Debug, Serialize, Deserialize)]
pub enum AudioQuality {
    CdRes,
    HiRes,
    NormalRes,
}
impl AudioQuality {
    pub fn as_str(&self) -> &str {
        match self {
            AudioQuality::CdRes => "CD-Res",
            AudioQuality::HiRes => "Hi-Res",
            AudioQuality::NormalRes => "Norm-Res",
        }
    }
}



#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Album {
    pub quality: AudioQuality,
    pub title: String,
    pub id: Option<String>,
    pub checksum: String,
}



#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct AlbumSet {
    pub albums: Vec<Album>,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct Music {
    pub single_album: Vec<Album>,
    pub album_set: Vec<AlbumSet>,
}

impl Music {
    pub fn new() -> Music {
        Music {
            single_album: Vec::new(),
            album_set: Vec::new(),
        }
    }
}

pub enum DirectoryType { // n pub
    Album {quality: AudioQuality, title: String, id: Option<String>, },
    AlbumSet {title: String},
}

pub async fn parse_directory(filename: &str) -> Result<DirectoryType, Error> { // n pub
    let path_strs = 
    filename
    .replace("][", "|")
    .replace("]", "|")
    .replace("[", "");
    let mut path_strs = path_strs.split('|');
    match path_strs.next() {
        Some("Set") => {
            let album_set_title = match path_strs.next() {
                None => return Err(Error::new(ErrorKind::Other, "Wrong set name.")),
                Some(title) => String::from(title),
            };
            Ok(DirectoryType::AlbumSet { title: album_set_title })
        },
        Some(album_id) => {
            let album_quality = match path_strs.next() {
                None => return Err(Error::new(ErrorKind::Other, "Audio quality not found")),
                Some(quality) => quality,
            };
            let album_title = match path_strs.next() {
                None => return Err(Error::new(ErrorKind::Other, "Album title not found")),
                Some(title) => title,
            };
            let album_quality = match album_quality {
                "Hi-Res" => AudioQuality::HiRes,
                "CD-Res" => AudioQuality::CdRes,
                "Norm-Res" => AudioQuality::NormalRes,
                _ => return Err(Error::new(ErrorKind::Other, "Audio quality parse error.")),
            };
            let album_id = match album_id {
                "N" => None,
                _ => Some(String::from(album_id)),
            };
            Ok(DirectoryType::Album { 
                    quality: album_quality,
                    title: String::from(album_title),
                    id: album_id,
                })
        },
        _ => Err(Error::new(ErrorKind::Other, "Folder name parse error.")),
    }


}

pub async fn blake3_dir_digest(dir: &Path) -> Result<Hash, Error> {
    if !dir.is_dir() {
        return Err(Error::new(ErrorKind::Other, "Path is not a directory."));
    }
    let mut b3hasher = blake3::Hasher::new();
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        if entry.path().is_dir() { continue; }
        let input = File::open(entry.path())?;
        let mut reader = BufReader::new(input);
        let mut buffer = [0; 4096];
        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 { break; }
            b3hasher.update(&buffer[..count]);
        }
    }
    Ok(b3hasher.finalize())
}


pub async fn scan(path: &Path) -> Result<Music, Error>
{
    let mut music = Music::new();
    if !path.is_dir() {
        return Err(Error::new(ErrorKind::Other, "Path is not a directory."));
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            return Err(Error::new(ErrorKind::Other, "Files in the root path is not allowed."));
        }
        let path_str = match path.to_str() {
            Some(pstr) => pstr,
            None => return Err(Error::new(ErrorKind::Other, "None name found.")),
        };
        let folder_name = match path.file_name() {
            Some(folder_name) => {
                match folder_name.to_str() {
                    Some(folder_name) => folder_name,
                    None => return Err(Error::new(ErrorKind::Other, "Folder name parse error."))
                }
            },
            None => return Err(Error::new(ErrorKind::Other, "Folder name parse error.")),
        };
        
        match parse_directory(folder_name).await? {
            DirectoryType::Album { quality, title, id} => {
                let album = Album {
                    quality,
                    title,
                    id,
                    checksum: blake3_dir_digest(path.as_path()).await?.to_string(),
                };
                music.single_album.push(album);
            },
            DirectoryType::AlbumSet { title } => {
                let mut album_set = AlbumSet {
                    albums: Vec::new(),
                    title,
                };
                for entry in fs::read_dir(path)? {
                    let entry = entry?;
                    let path = entry.path();
                    let path_str = match path.to_str() {
                        Some(s) => s,
                        None => return Err(Error::new(ErrorKind::Other, "Album set inside parse error.")),
                    };
                    let folder_name = match path.file_name() {
                        Some(folder_name) => {
                            match folder_name.to_str() {
                                Some(folder_name) => folder_name,
                                None => return Err(Error::new(ErrorKind::Other, "Folder name parse error."))
                            }
                        },
                        None => return Err(Error::new(ErrorKind::Other, "Folder name parse error.")),
                    };
                    match parse_directory(folder_name).await? {
                        DirectoryType::Album { quality, title, id } => {
                            let album = Album {
                                quality,
                                title,
                                id,
                                checksum: blake3_dir_digest(path.as_path()).await?.to_string(),
                            };
                            album_set.albums.push(album);
                        },
                        _ => return Err(Error::new(ErrorKind::Other, "Not a album in the album set.")),

                    }
                }
                music.album_set.push(album_set);
            },

        }

    }

    Ok(music)
}


