use std::path::Path;

use music_manager::scanner::DirectoryType;
use octocrab::{Octocrab, models::issues::Issue, Error};
use blake3;
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
    let p = Path::new("D:/hash_test");
    let t = scanner::blake3_dir_digest(p).await;
    println!("{}", t.unwrap().to_hex());


}
