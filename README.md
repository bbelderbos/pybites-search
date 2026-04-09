# Pybites Search

This is a simple command line tool to search through all Pybites content: articles, bite exercises, tips, podcasts and YouTube videos.

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

$ psearch rust
... quite a lot of output ...

$ psearch rust -c v (or -c video)
... only videos ...


$ psearch rust -c a (or -c article)
... only articles ...

$ psearch dataclass -c t
... only dataclass tips ...

$ psearch counter -c b -t
... only bite exercises with title matching counter ...

$ psearch transpose data
... strings this together into a regex matching "[bite] Transpose a data structure" for example ...
```
