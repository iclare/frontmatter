use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;

use regex::Regex;

use lazy_static::lazy_static;

fn parse(toks: &str) -> HashSet<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\w(-?\w?)*").unwrap();
    }
    RE.find_iter(toks)
        .map(|tag| String::from(tag.as_str()))
        .collect()
}

pub fn extract(file: &PathBuf) -> HashSet<String> {
    let output = &Command::new("mdls")
        .args(&[
            "-raw",
            "-name",
            "kMDItemOMUserTags",
            &file.to_string_lossy(),
        ])
        .output()
        .unwrap_or_else(|_| panic!("error: could not read tags from {:?}", file))
        .stdout;
    let tags_raw = &String::from_utf8_lossy(&output).to_string();
    parse(tags_raw)
}
