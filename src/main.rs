#![deny(unsafe_code)]
#![warn(missing_docs)]

//! Bot for automating tasks in the Rust wiki
//!
//! This is WIP and currently doesn't do anything useful.
//! It just fetches all pages that include a certain template and prints them.

use mediawiki::api::Api;
use std::{error::Error, time::Instant};

use clap::Clap;

#[macro_use]
mod macros;

const DEFAULT_API_URL: &str = "https://runrust.miraheze.org/w/api.php";
const DEFAULT_USER: &str = "Dev-WikiBot";
const PAGE_ID: &str = "97";

/// 🚧 WIP 🚧
///
/// This currently just requests the page with the ID 97 and prints all pages
/// where this page is transcluded.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();

    let time = Instant::now();

    let lgname = opts.username;
    let lgpassword = opts.password;

    println!("Logging in as {}", lgname);

    let mut api = Api::new(&opts.api_url).await?;
    api.set_user_agent("wikibot");
    api.login(lgname, lgpassword).await?;

    println!("==> Login finished in {:.2?}", time.elapsed());
    let time = Instant::now();

    println!("Requesting page ID {}", PAGE_ID);

    let params = map!(action: "query", prop: "transcludedin", pageids: PAGE_ID);
    let result = api.query_api_json(&params, Method::Get).await?;
    let page = &result["query"]["pages"][PAGE_ID];

    println!("==> Request finished in {:.2?}", time.elapsed());

    println!("Page: {}", page["title"]);
    println!("Transcluded in:");

    for value in page["transcludedin"].as_array().unwrap() {
        println!(
            "{:>5}  {}",
            value["pageid"].as_u64().unwrap(),
            value["title"]
        );
    }

    Ok(())
}

#[derive(Clap)]
struct Opts {
    /// Usually "https://<host>/w/api.php"
    #[clap(long, env, default_value=DEFAULT_API_URL)]
    api_url: String,
    /// The username of the bot account
    #[clap(long, env, default_value=DEFAULT_USER)]
    username: String,
    /// The password of the bot account
    #[clap(long, env)]
    password: String,
}

#[cfg(test)]
mod tests {
    use std::env;

    /// To check if password is set in CI
    #[test]
    fn test_password_set() {
        assert!(env::var("PASSWORD").is_ok());
    }
}
