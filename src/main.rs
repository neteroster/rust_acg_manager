use std::path::Path;

use music_manager::scanner::{Music, Album, AlbumSet};
use octocrab::{Octocrab, models::issues::Issue, Error};
use study::{music_manager::scanner, config::parser::AllConfig};
mod music_manager;
mod config;


const TOKEN : &str = "";
async fn init_issue(ghc: &Octocrab) -> Result<Issue, Error> {
    let iss = ghc
    .issues("neteroster", "blog")
    .create("ACGN Auto Managing Program [Test] (rust)")
    .body("This is a test issue sent by rust.")
    .send()
    .await?;
    Ok(iss)
}

#[tokio::main]
async fn main() {
    let p = Path::new("D:/ShortTermTemp/Music-5F0455E09EA42C457E17F6997C89CD9D74E40A026B3FD57F8E8C9A93704D1EE7");
    let t = scanner::scan(p).await.unwrap();
    let r = t.to_markdown();
    print!("{}", r.as_str());
    let cfgp = Path::new("D:/cfs.json");
    let res = AllConfig::from_file(cfgp).unwrap();
    t.push_to_github(&res).await.unwrap();

}
