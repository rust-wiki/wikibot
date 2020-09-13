#![deny(unsafe_code)]
#![warn(missing_docs)]

//! Bot for automating tasks in the Rust wiki
//!
//! This is WIP and currently doesn't do anything useful.

use std::{error::Error, time::Instant};

use clap::Clap;
use mediawiki::api::Api;
use mediawiki::method::Method;

#[macro_use]
mod macros;

const PAGE_ID: &str = "56"; // Template:CrateLink

/// 🚧 WIP 🚧
///
/// This currently just requests the page with the ID 56 and prints all transclusions of this page.
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
    let result = api.query_api_json(params, Method::Get).await?;
    let page = &result["query"]["pages"][PAGE_ID];

    let template_name = page["title"]
        .as_str()
        .unwrap()
        .trim_start_matches("Template:");

    println!("==> Request finished in {:.2?}", time.elapsed());

    println!("Page: {}", page["title"]);
    println!("Transcluded in:");

    for value in page["transcludedin"].as_array().unwrap() {
        println!(
            "{:>5}  {}",
            value["pageid"].as_u64().unwrap(),
            value["title"]
        );

        if let Some(page_id) = value["pageid"].as_u64() {
            let params = map!(action: "parse", prop: "parsetree", pageid: page_id.to_string());
            let result = api.query_api_json(params, Method::Get).await?;
            let parse_tree = result["parse"]["parsetree"]["*"].as_str().unwrap();
            let parse_tree = roxmltree::Document::parse(parse_tree)?;
            for node in parse_tree.descendants() {
                if node.tag_name().name() == "template" {
                    if node
                        .children()
                        .find(|child| child.tag_name().name() == "title")
                        .and_then(|title_tag| title_tag.text())
                        .map(|title| title == template_name)
                        .unwrap_or(false)
                    {
                        if let Some(part) = node
                            .children()
                            .find(|child| child.tag_name().name() == "part")
                        {
                            if let Some(crate_name) = part
                                .children()
                                .find(|child| child.tag_name().name() == "name")
                                .and_then(|name_tag| {
                                    name_tag
                                        .next_siblings()
                                        .find(|sib| sib.tag_name().name() == "value")
                                })
                                .and_then(|crate_name| crate_name.text())
                            {
                                println!("{:>10}{{{{CrateLink|{}}}}}", "", crate_name);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

#[derive(Clap)]
struct Opts {
    /// Usually "https://<host>/w/api.php"
    #[clap(long, env, default_value = "https://runrust.miraheze.org/w/api.php")]
    api_url: String,
    /// The username of the bot account
    #[clap(long, env, default_value = "Dev-WikiBot")]
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
