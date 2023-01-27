//! Utility to make use of the dimension library.

use ahash::AHashSet;
use clap::{Args, Parser, Subcommand, ValueEnum};
use dimensions::stub::Dimension as StubDimension;
use dimensions::{Dimension, ResourceName};
use indicatif::{ProgressBar, ProgressStyle};
use regex::bytes::Regex;
use serde_json;
use std::fs::File;
use std::io::{self, BufWriter, Stdout, Write};
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum Format {
    Pretty,
    Json,
    JsonPretty,
    Debug,
    Compact,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(short, long, default_value = "pretty")]
    /// Format output according to the given formatter
    format: Format,

    #[arg(short, long)]
    /// Output file to write to
    out: Option<PathBuf>,
}

fn print_dim(dim: &Dimension, writer: &mut BufWriter<&mut dyn Write>, format: Format) {
    match format {
        Format::Pretty => writeln!(writer, "{}", dim).unwrap(),
        Format::Json => {
            serde_json::to_writer(&mut *writer, dim).unwrap();
            writer.write_all(b"\n").unwrap();
        }
        Format::JsonPretty => {
            serde_json::to_writer_pretty(&mut *writer, dim).unwrap();
            writer.write_all(b"\n").unwrap();
        }
        Format::Debug => writeln!(writer, "{:#?}", dim).unwrap(),
        Format::Compact => dim.write_compact(writer).unwrap(),
    }
}

#[derive(Subcommand)]
enum Command {
    /// Show values for a specific dimension, with "x y" coords
    Show(Show),

    /// Search for specific dimensions
    Search(Search),
}

#[derive(Args)]
struct Show {
    #[arg(required = true, allow_negative_numbers = true)]
    #[arg(num_args = 2, value_names(["X", "Y"]))]
    show: Vec<i32>,
}

#[derive(Args)]
struct Search {
    #[arg(long, allow_negative_numbers = true)]
    #[arg(num_args = 2, value_names(["X", "Y"]))]
    #[arg(requires = "coord_max")]
    /// Minimum bound for coords
    coord_min: Vec<i32>,

    #[arg(long, allow_negative_numbers = true)]
    #[arg(num_args = 2, value_names(["X", "Y"]))]
    #[arg(requires = "coord_min")]
    /// Maximum bound for coords, inclusive
    coord_max: Vec<i32>,

    #[arg(short, long)]
    #[arg(default_value = "")]
    /// Regex that the dimension name must match. Implicitly includes starting/ending ^$
    name: String,
}

fn make_progress(size: u64) -> ProgressBar {
    ProgressBar::new(size).with_style(
        ProgressStyle::with_template("{elapsed_precise}{wide_bar}{eta_precise} {pos:>7}/{len:7}")
            .unwrap()
            .progress_chars("=>·"),
    )
}

fn main() {
    let args = Cli::parse();
    let mut stdout: Stdout;
    let mut file: File;
    let mut writer: BufWriter<&mut dyn Write> = BufWriter::new(match args.out {
        None => {
            stdout = io::stdout();
            &mut stdout
        }
        Some(path) => {
            file = File::create(path).unwrap();
            &mut file
        }
    });
    match &args.command {
        Command::Show(Show { show }) => {
            if let [x, y] = show[..] {
                let dim = Dimension::new(x, y);
                print_dim(&dim, &mut writer, args.format);
            } else {
                panic!("No args in {show:?}");
            }
        }
        Command::Search(search) => {
            let mut hash = AHashSet::<u32>::with_capacity(512);
            if !search.name.is_empty() {
                let re =
                    Regex::new(&format!("(?i-u)^{}$", search.name)).expect("Failed to parse name");
                eprintln!("Finding matching names...");
                let progress = make_progress(1 << 24);
                let mut i = 0u32;
                while i < (1 << 24) {
                    progress.set_position(i.into());
                    for _ in 0..(1 << 16) {
                        ResourceName::filter(&re, i, &mut hash);
                        i += 1
                    }
                }
                progress.finish_and_clear();
                eprintln!("Found {} matching names.\n", hash.len());
                if hash.is_empty() {
                    return;
                }
            }

            let [x_min, y_min] = match search.coord_min[..] {
                [x, y] => [x, y],
                _ => [-60000, -60000],
            };
            let [x_max, y_max] = match search.coord_max[..] {
                [x, y] => [x, y],
                _ => [60000, 60000],
            };
            let y_total = u64::try_from(y_max - y_min + 1).unwrap();
            let x_total = u64::try_from(x_max - x_min + 1).unwrap();
            let progress = make_progress(y_total * x_total);
            for y in y_min..=y_max {
                if hash.is_empty() {
                    // Raw loop, no filtering
                    for x in x_min..=x_max {
                        let dim = Dimension::new(x, y);
                        print_dim(&dim, &mut writer, args.format);
                    }
                } else {
                    // Use filtered results by testing a stub dimension first
                    for x in x_min..=x_max {
                        let stub = StubDimension::new(x, y);
                        if !hash.contains(&stub.name) {
                            continue;
                        }
                        let dim = Dimension::new(x, y);
                        print_dim(&dim, &mut writer, args.format);
                    }
                }
                progress.inc(x_total);
            }
        }
    }
    writer.flush().expect("Failed to finalize file");
}
