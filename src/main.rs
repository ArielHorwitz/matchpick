use anyhow::{Context, Result};
use clap::Parser;
use matchpick::process;
use std::{io::Read, path::PathBuf};

const START_PATTERN: &str = "~>>>";
const END_PATTERN: &str = "~<<<";

#[derive(Debug, Parser)]
#[clap(name = "matchpick")]
#[clap(about = "Find and replace multi-lines using a match-case")]
#[clap(author = "https://ariel.ninja")]
#[clap(version)]
struct Args {
    /// Read from file (otherwise from stdin)
    #[arg()]
    file: Option<PathBuf>,
    /// Match against (switch case)
    #[arg(short, long = "match")]
    match_against: Option<String>,
    /// Pattern to start matching and switch cases
    #[arg(short = 's', long, default_value_t = String::from(START_PATTERN))]
    start_pattern: String,
    /// Pattern to end matching
    #[arg(short = 'e', long, default_value_t = String::from(END_PATTERN))]
    end_pattern: String,
    /// Pattern to ignore other patterns
    #[arg(long)]
    ignore_pattern: Option<String>,
    /// Output to file (otherwise to stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,
    /// Print default starting pattern
    #[arg(long)]
    print_start: bool,
    /// Print default ending pattern
    #[arg(long)]
    print_end: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    // shortcuts
    if args.print_start {
        println!("{START_PATTERN}");
        return Ok(());
    };
    if args.print_end {
        println!("{END_PATTERN}");
        return Ok(());
    };
    // get data
    let input_data = if let Some(file) = &args.file {
        std::fs::read(file).context("read input file")?
    } else {
        let mut data = Vec::new();
        let mut stdin = std::io::stdin().lock();
        stdin.read_to_end(&mut data).context("read stdin")?;
        data
    };
    let utf8_data = String::from_utf8(input_data).context("parse utf8")?;
    let output_data = process(
        &utf8_data,
        args.match_against,
        &args.start_pattern,
        &args.end_pattern,
        args.ignore_pattern,
    )?;
    if let Some(output_file) = args.output {
        std::fs::write(&output_file, &output_data).context("write to file")?;
    } else {
        println!("{output_data}");
    }
    Ok(())
}
