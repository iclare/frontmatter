use std::path::PathBuf;
use std::process::Command;

use structopt::StructOpt;

use crate::frontmatter::Frontmatter;

mod frontmatter;

#[derive(StructOpt, Debug)]
#[structopt(name = "FRONTMATTER")]
struct Cli {
    /// Verbose mode
    #[structopt(short, long)]
    verbose: bool,
    /// Output directory
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,
    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn parse_args() -> Cli {
    let args = Cli::from_args();

    let verbose: bool = args.verbose;
    let files: Vec<PathBuf> = args.files;
    let output: PathBuf = args.output;

    if !output.is_dir() {
        eprintln!("error: not a directory {:?}", output);
        std::process::exit(1);
    }

    for file in &files {
        if !file.is_file() {
            eprintln!("error: not a file {:?}", file);
            std::process::exit(1);
        }
    }

    Cli {
        verbose,
        output,
        files,
    }
}

fn main() -> std::io::Result<()> {
    Command::new("which")
        .arg("mdls")
        .output()
        .unwrap_or_else(|_| panic!("error: could not find mdls"));

    let Cli {
        verbose,
        files,
        output,
    } = parse_args();

    for file in files {
        let Frontmatter {
            title,
            updated,
            date,
            tags,
        } = frontmatter::extract(&file).unwrap();

        let frontmatter = format!("\
---
title: {:?}
updated: {:?}
date: {:?}
tags: {:?}
---\
", title, updated, date, tags);

        if verbose {
            println!("{}", frontmatter)
        }
    }

    Ok(())
}
