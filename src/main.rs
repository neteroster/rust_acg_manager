use music_manager::scanner::DirectoryType;
use octocrab::{Octocrab, models::issues::Issue, Error};
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
    let t = music_manager::scanner::parse_directory("[Set]Angel Beats!").await.unwrap();
    match t {
        DirectoryType::AlbumSet { title: b } => println!("{}", b),
        _ => (),
    }

}
