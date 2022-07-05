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
//impl Music {
//    pub fn to_markdown() -> Result<String, Error> {
//        let mut res_str = String::new();
//
//    }
//}

