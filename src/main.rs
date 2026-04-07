use clap::{CommandFactory, Parser};
use colored::*;
use phf::phf_map;
use regex::Regex;
use serde::Deserialize;

const CONTENT_DATA: &str = include_str!("../data/content.json");

static CATEGORY_MAPPING: phf::Map<&'static str, &'static str> = phf_map! {
    "a" => "article",
    "b" => "bite",
    "p" => "podcast",
    "v" => "video",
    "t" => "tip",
};

#[derive(Deserialize, Debug)]
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

fn main() {
    let cli = Cli::parse();

    if cli.search_terms.is_empty() {
        eprintln!(
            "{}",
            "Error: At least one search term should be given.".red()
        );
        Cli::command().print_help().ok();
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

    let items: Vec<Item> = serde_json::from_str(CONTENT_DATA).expect("Invalid embedded JSON");

    search_items(&items, &search_term, content_type, cli.title_only);
}

fn search_items(items: &[Item], search_term: &str, content_type: Option<&str>, title_only: bool) {
    let re = Regex::new(&format!("(?i){}", search_term)).unwrap();

    for item in items {
        let matches = if title_only {
            re.is_match(&item.title)
        } else {
            re.is_match(&item.title) || re.is_match(&item.summary)
        };
        if content_type.is_none_or(|t| t.eq_ignore_ascii_case(&item.content_type)) && matches {
            let content_type_prefix: String = if content_type.is_none() {
                format!("[{}] ", item.content_type)
            } else {
                "".to_string()
            };
            println!("{}{}\n{}\n", content_type_prefix, item.title, item.link);
        }
    }
}
