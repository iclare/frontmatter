use std::path::PathBuf;

use structopt::StructOpt;

use tags::extract;

mod tags;

#[derive(StructOpt, Debug)]
#[structopt(name = "frontmatter")]
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

#[allow(unused)]
fn main() -> std::io::Result<()> {
    let args = Cli::from_args();

    let verbose: bool = args.verbose;
    let files: &Vec<PathBuf> = &args.files;
    let output: &PathBuf = &args.output;

    if !output.is_dir() {
        eprintln!("error: not a directory {:?}", output);
        std::process::exit(1);
    }

    for file in files {
        if !file.is_file() {
            eprintln!("error: not a file {:?}", file);
            std::process::exit(1);
        }
        let tags = extract(&file);
        // println!("{:?}", tags);
    }

    Ok(())
}
