use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// Verbose mode
    #[structopt(short, long)]
    verbose: bool,
    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let args = Cli::from_args();
    let verbose: bool = args.verbose;
    let files: &Vec<PathBuf> = &args.files;

    for file in files {
        if verbose {
            println!("{:?}", file);
        }
        let file_stem = match file.file_stem() {
            Some(file_stem) => file_stem,
            None => {
                eprintln!("error: not a file {:?}", file);
                std::process::exit(1);
            }
        };
    }
}
