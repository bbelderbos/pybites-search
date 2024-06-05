use cached::proc_macro::cached;
use reqwest;
use serde::Deserialize;
use regex::Regex;
use std::env;
use std::time::Duration;

const TIMEOUT: u64 = 10;
const ENDPOINT: &str = "https://codechalleng.es/api/content/";

#[derive(Deserialize, Debug, Clone)]
struct Item {
    content_type: String,
    title: String,
    summary: String,
    link: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 4 {
        eprintln!("Usage: search <search_term> [<content_type>] [--title-only]");
        return Ok(());
    }

    let search_term = &args[1];
    let content_type = if args.len() >= 3 && !args[2].starts_with("--") { Some(&args[2]) } else { None };
    let title_only = args.contains(&"--title-only".to_string());

    let items = match fetch_items(ENDPOINT.to_string()).await {
        Ok(items) => items,
        Err(e) => {
            eprintln!("Error fetching items: {:?}", e);
            return Err(e.into());
        }
    };

    search_items(&items, search_term, content_type.map(String::as_str), title_only);

    Ok(())
}

#[cached(time = 600, result = true, sync_writes = true)]
async fn fetch_items(endpoint: String) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(&endpoint)
        .timeout(Duration::from_secs(TIMEOUT))
        .send()
        .await?
        .error_for_status()? // Ensure the response status is a success
        .json::<Vec<Item>>()
        .await?;
    Ok(response)
}

fn search_items(items: &[Item], search_term: &str, content_type: Option<&str>, title_only: bool) {
    let re = Regex::new(&format!("(?i){}", regex::escape(search_term))).unwrap();

    for item in items {
        let matches = if title_only {
            re.is_match(&item.title)
        } else {
            re.is_match(&item.title) || re.is_match(&item.summary)
        };
        if content_type.map_or(true, |t| t.eq_ignore_ascii_case(&item.content_type)) && matches {
            if content_type.is_none() {
                println!("Type: {}", item.content_type);
            }
            println!("Title: {}", item.title);
            println!("Link: {}\n", item.link);
        }
    }
}

