# Pybites CLI Search

This is a simple command line tool to search through Pybites content: articles, bite exercises, tips, podcasts and YouTube videos.

## Installation

```bash
cargo install pybites-search
```

## Usage

```bash
$ psearch --version
psearch 1.0.0
$ psearch --help
A command-line search tool for Pybites content

Usage: psearch [OPTIONS] [SEARCH_TERMS]...

Arguments:
  [SEARCH_TERMS]...

Options:
  -c, --content-type <CONTENT_TYPE>
  -t, --title-only
  -h, --help                         Print help
  -V, --version                      Print version
```

## 1.0.0 change

If you're still on 0.6.0 you now get:

```bash
$ psearch command -c v
Cache expired, fetching latest data from API ...
Error: reqwest::Error { kind: Request, url: Url { scheme: "https", cannot_be_a_base: false, username: "", password: None, host: Some(Domain("codechalleng.es")), port: None, path: "/api/content/", query: None, fragment: None }, source: TimedOut }
```

This is because the codechalleng.es API endpoint is no longer live. Content meta data is now stored locally in the binary. So if you're on < 1.0.0, just upgrade and it should work again:

```bash
$ cargo install pybites_install --force
...
...
    Replaced package `pybites-search v0.6.0` with `pybites-search v1.0.0` (executable `psearch`)

$ psearch command -c v
Finding Pybites Content Quickly With Our Search Command Line Tool
https://www.youtube.com/watch?v=0zROYGLTFKA

Debugging a failing Heroku Django command
https://www.youtube.com/watch?v=8HfGL8fo_58
...
...
```
