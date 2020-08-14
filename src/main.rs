use mediawiki::api::Api;

#[macro_use]
mod macros;

const API_URL: &str = "https://runrust.miraheze.org/w/api.php";
const PAGE_ID: &str = "97";

/// 🚧 WIP 🚧
///
/// This currently just requests the page with the ID 97 and prints all pages
/// where this page is transcluded.
#[tokio::main]
async fn main() {
    let api = Api::new(API_URL).await.expect("Could not initialize API");

    let params = map!(action: "query", prop: "transcludedin", pageids: PAGE_ID);
    let result = api.query_api_json(&params, "GET").await.unwrap();
    let page = &result["query"]["pages"][PAGE_ID];

    println!("Page: {}", page["title"]);
    println!("Page ID: {}", page["pageid"]);
    println!("Transcluded in:");

    for value in page["transcludedin"].as_array().unwrap() {
        println!(
            "{:>5}  {}",
            value["pageid"].as_u64().unwrap(),
            value["title"]
        );
    }
}
