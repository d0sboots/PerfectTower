//! Utility to make use of the dimension library.

use std::io::{self, Write};
use clap::{Args, Parser, Subcommand, ValueEnum};
use dimensions::Dimension;
use serde_json;

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum Format {
    Pretty,
    Json,
    JsonPretty,
    Debug,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(short, long, default_value = "pretty")]
    /// Format output according to the given formatter
    format: Format,
}

fn print_dim(dim: &Dimension, format: Format) {
    let mut handle = io::stdout().lock();
    match format {
        Format::Pretty => writeln!(handle, "{}", dim).unwrap(),
        Format::Json => {
            serde_json::to_writer(&mut handle, dim).unwrap();
            handle.write_all(b"\n").unwrap();
        },
        Format::JsonPretty => {
            serde_json::to_writer_pretty(io::stdout(), dim).unwrap();
            handle.write_all(b"\n").unwrap();
        }
        Format::Debug => writeln!(&mut handle, "{:#?}", dim).unwrap(),
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
    #[arg(allow_negative_numbers = true)]
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
}

fn main() {
    let args = Cli::parse();
    match &args.command {
        Command::Show(Show { show }) => {
            if let [x, y] = show[..] {
                let dim = Dimension::new(x, y);
                print_dim(&dim, args.format);
            } else {
                panic!("No args in {show:?}");
            }
        }
        Command::Search(search) => {
            let [x_min, y_min] = match search.coord_min[..] {
                [x, y] => [x, y],
                _ => [-60000, -60000],
            };
            let [x_max, y_max] = match search.coord_max[..] {
                [x, y] => [x, y],
                _ => [60000, 60000],
            };
            for y in y_min..=y_max {
                for x in x_min..=x_max {
                    let dim = Dimension::new(x, y);
                    print_dim(&dim, args.format);
                }
            }
        }
    }
}
