use std::collections::HashSet;
use std::iter::FromIterator;
use std::path::PathBuf;
use std::process::Command;

use regex::Regex;

use lazy_static::lazy_static;

/// Ex:
///     from - "( atag, another-tag )"
///     to - {"atag", "another-tag"}
fn parse_tags(toks: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\w(-?\w?)*").unwrap();
    }
    let tags: HashSet<String> = RE.find_iter(toks)
        .map(|tag| String::from(tag.as_str()).to_lowercase())
        .filter(|tag| tag != "null")
        .collect();
    Vec::from_iter(tags)
}

/// Ex:
///     from - "2020-04-16 01:32:44 +0000"
///     to - "2020-04-16"
fn parse_date(date: &str) -> String {
    String::from(date.split(" ").collect::<Vec<&str>>()[0])
}

pub struct Frontmatter {
    pub title: String,
    pub updated: String,
    pub date: String,
    pub tags: Vec<String>,
}

pub fn extract(file: &PathBuf) -> Option<Frontmatter> {
    lazy_static! {
        static ref WS: Regex = Regex::new(r"\s+").unwrap();
    }
    let raw_output = &Command::new("mdls")
        .args(&[
            "-name",
            "kMDItemFSContentChangeDate",
            "-name",
            "kMDItemFSCreationDate",
            "-name",
            "kMDItemOMUserTags",
            &file.to_string_lossy(),
        ])
        .output()
        .unwrap_or_else(|_| panic!("error: could not read metadata from {:?}", file))
        .stdout;
    let rawline = &String::from_utf8_lossy(raw_output).to_string();
    let nls = &WS.replace_all(rawline, " ");
    let splits = &nls.split(" = ").collect::<Vec<&str>>();

    let title = String::from(file.file_stem()?.to_str()?);
    let updated = parse_date(splits[1]);
    let date = parse_date(splits[2]);
    let tags = parse_tags(splits[3]);

    Some(Frontmatter {
        title,
        updated,
        date,
        tags,
    })
}
