use std::collections::LinkedList;
use std::fs::{DirEntry, File};
use std::io::{BufReader, Read};
use std::{path::Path, io::Error, io::ErrorKind};
use std::{fs, path};
use blake3::{self, Hash};
use walkdir::{self, WalkDir};

pub enum AudioQuality {
    CdRes,
    HiRES,
    NormalRes,
}

pub enum CheckSum {
    Sha256(String),
    Blake3(String),
}

#[allow(dead_code)]

#[allow(dead_code)]
pub struct Album {
    quality: AudioQuality,
    title: String,
    id: String,
    checksum: CheckSum,
}

#[allow(dead_code)]
pub struct AlbumSet {
    albums: Vec<Album>,
    title: String,
}

pub struct Music {
    single_album: Vec<Album>,
    album_set: Vec<AlbumSet>,
}

pub enum DirectoryType { // n pub
    Album {quality: AudioQuality, title: String, id: String, },
    AlbumSet {title: String},
}

pub async fn parse_directory(path_str: &str) -> Result<DirectoryType, Error> { // n pub
    let path_strs = 
    path_str
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
                "Hi-Res" => AudioQuality::HiRES,
                "CD-Res" => AudioQuality::CdRes,
                "Norm-Res" => AudioQuality::NormalRes,
                _ => return Err(Error::new(ErrorKind::Other, "Audio quality parse error.")),
            };
            Ok(DirectoryType::Album { 
                    quality: album_quality,
                    title: String::from(album_title),
                    id: String::from(album_id),
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
    let mut music : Music;
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

    }

    Err(Error::new(ErrorKind::Other, "Pass"))
}


