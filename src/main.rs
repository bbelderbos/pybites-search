use clap::{CommandFactory, Parser};
use colored::*;
use dirs::home_dir;
use phf::phf_map;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const TIMEOUT: u64 = 10;
const ENDPOINT: &str = "https://codechalleng.es/api/content/";
const CACHE_FILE_NAME: &str = ".pybites-search-cache.json";
const DEFAULT_CACHE_DURATION: u64 = 86400; // Cache API response for 1 day

static CATEGORY_MAPPING: phf::Map<&'static str, &'static str> = phf_map! {
    "a" => "article",
    "b" => "bite",
    "p" => "podcast",
    "v" => "video",
    "t" => "tip",
};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Item {
    content_type: String,
    title: String,
    summary: String,
    link: String,
}

#[derive(Parser)]
#[command(name = "psearch", version, about)]
struct Cli {
    search_terms: Vec<String>,

    #[arg(short = 'c', long = "content-type")]
    content_type: Option<String>,

    #[arg(short = 't', long = "title-only")]
    title_only: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.search_terms.is_empty() {
        eprintln!(
            "{}",
            "Error: At least one search term should be given.".red()
        );
        Cli::command().print_help()?;
        std::process::exit(1);
    }

    let search_term = cli
        .search_terms
        .iter()
        .map(|term| regex::escape(term))
        .collect::<Vec<_>>()
        .join(".*");

    let ct = cli.content_type.as_deref();
    let content_type = match ct {
        Some(ct) => {
            if CATEGORY_MAPPING.get(ct).is_some() || CATEGORY_MAPPING.values().any(|&v| v == ct) {
                CATEGORY_MAPPING.get(ct).cloned().or(Some(ct))
            } else {
                let valid_options: Vec<String> = CATEGORY_MAPPING
                    .entries()
                    .map(|(key, value)| format!("{} ({})", key, value))
                    .collect();
                eprintln!(
                    "Error: Invalid content type '{}'. Valid options are: {}",
                    ct,
                    valid_options.join(", ")
                );
                std::process::exit(1);
            }
        }
        None => None,
    };

    let title_only = cli.title_only;

    let cache_duration = env::var("CACHE_DURATION")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_CACHE_DURATION);

    let items = fetch_items(ENDPOINT.to_string(), cache_duration).await?;

    search_items(&items, &search_term, content_type, title_only);

    Ok(())
}

async fn fetch_items(
    endpoint: String,
    cache_duration: u64,
) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    if let Ok(items) = load_from_cache(cache_duration) {
        return Ok(items);
    }

    println!(
        "{}",
        "Cache expired, fetching latest data from API ...".yellow()
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&endpoint)
        .timeout(Duration::from_secs(TIMEOUT))
        .send()
        .await?
        .error_for_status()? // Ensure the response status is a success
        .json::<Vec<Item>>()
        .await?;

    save_to_cache(&response)?;

    Ok(response)
}

fn save_to_cache(items: &[Item]) -> Result<(), Box<dyn std::error::Error>> {
    let cache_path = get_cache_file_path();
    let cache_data = CacheData {
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        items: items.to_vec(),
    };
    let serialized = serde_json::to_string(&cache_data)?;
    fs::write(cache_path, serialized)?;
    Ok(())
}

fn load_from_cache(cache_duration: u64) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    let cache_path = get_cache_file_path();
    let data = fs::read_to_string(cache_path)?;
    let cache_data: CacheData = serde_json::from_str(&data)?;

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    if current_time - cache_data.timestamp <= cache_duration {
        Ok(cache_data.items)
    } else {
        Err("Cache expired".into())
    }
}

#[derive(Deserialize, Serialize)]
struct CacheData {
    timestamp: u64,
    items: Vec<Item>,
}

fn search_items(items: &[Item], search_term: &str, content_type: Option<&str>, title_only: bool) {
    let re = Regex::new(&format!("(?i){}", search_term)).unwrap();

    for item in items {
        let matches = if title_only {
            re.is_match(&item.title)
        } else {
            re.is_match(&item.title) || re.is_match(&item.summary)
        };
        if content_type.map_or(true, |t| t.eq_ignore_ascii_case(&item.content_type)) && matches {
            let content_type_prefix: String = if content_type.is_none() {
                format!("[{}] ", item.content_type)
            } else {
                "".to_string()
            };
            println!("{}{}\n{}\n", content_type_prefix, item.title, item.link);
        }
    }
}

fn get_cache_file_path() -> PathBuf {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(CACHE_FILE_NAME);
    path
}
