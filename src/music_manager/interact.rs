use std::io::Error;
use super::scanner::{Music, Album, AlbumSet, AudioQuality};


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

impl Music {
    pub fn to_markdown(&self) -> String {
        let mut res_str = String::new();
        for album in &self.single_album {
            res_str.push_str(generate_line_album(&album, 0).as_str());
            res_str.push('\n');
        }
        for album_set in &self.album_set {
            res_str.push_str(format!("[Album Set] {}", album_set.title).as_str());
            res_str.push('\n');
            for album in &album_set.albums {
                res_str.push_str(generate_line_album(album, 1).as_str());
                res_str.push('\n');
            }
        }

        res_str
    }
}

