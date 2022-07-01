use octocrab::{Octocrab, models::issues::Issue, Error};


const TOKEN : &str = "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";
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
    let ghc_builder = octocrab::OctocrabBuilder::new();
    let ghc = ghc_builder.personal_token(String::from(TOKEN)).build().unwrap();
    match init_issue(&ghc).await {
        Err(err) => println!("{}", err.to_string()),
        _ => (),
    }

    println!("5");

}
