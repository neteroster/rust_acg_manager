use std::path::Path;

use music_manager::scanner::{Music, Album, AlbumSet};
use octocrab::{Octocrab, models::issues::Issue, Error};
use study::music_manager::scanner;
mod music_manager;


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
    let p = Path::new("D:/cd_test");
    let t = scanner::scan(p).await.unwrap();
    let r = t.to_markdown();
    print!("{}", r.as_str());
    

}
