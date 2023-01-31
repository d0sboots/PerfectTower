//! Utility to make use of the dimension library.

use ahash::AHashSet;
use clap::{Args, Parser, Subcommand, ValueEnum};
use dimensions::stub::Dimension as StubDimension;
use dimensions::{qty_to_int, round_ties_even, Dimension, ResourceName};
use indicatif::{ProgressBar, ProgressStyle};
use regex::bytes::Regex;
use serde_json;
use std::fs::File;
use std::io::{self, BufWriter, Stdout, Write};
use std::num::{ParseFloatError, ParseIntError};
use std::path::PathBuf;
use thiserror::Error;

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

    #[arg(long)]
    #[arg(default_value = "1")]
    /// Minimum number of resource stacks (shown as "Resources" in-game) per dimension
    stack_min: usize,

    #[arg(long)]
    #[arg(default_value = "3")]
    /// Maximum number of resource stacks (shown as "Resources" in-game) per dimension, inclusive
    stack_max: usize,

    #[arg(long)]
    #[arg(default_value = "any")]
    /// How to filter resource stacks. "all" means all stacks must meet the critera, "any" means
    /// at least one must.
    filter: FilterType,

    #[arg(long, value_parser = parse_qty)]
    #[arg(default_value = "0.001")]
    /// Minimum res/sec for resource stacks. Can also be specified in hex from 0x000000 to 0xffffff.
    qty_min: u32,

    #[arg(long, value_parser = parse_qty)]
    #[arg(default_value = "8.5")]
    /// Maximum res/sec for resource stacks. Can also be specified in hex from 0x000000 to 0xffffff. Inclusive.
    qty_max: u32,

    #[arg(long, default_value = "0.5")]
    /// Search for dimensions with a cost within <tolerance> of x.5. This stress-tests the
    /// generation algorithm, and is of little use otherwise.
    tolerance: f64,

    #[arg(long, default_value = "1")]
    /// Minimum cost for dimensions.
    cost_min: u32,

    #[arg(long, default_value = "5392")]
    /// Maximum cost for dimensions. Inclusive.
    cost_max: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum FilterType {
    All,
    Any,
}

#[derive(Error, Debug)]
#[error(transparent)]
enum ParseQtyError {
    Float(#[from] ParseFloatError),
    Int(#[from] ParseIntError),
}

fn parse_qty(value: &str) -> Result<u32, ParseQtyError> {
    if value.get(..2) == Some("0x") {
        Ok(u32::from_str_radix(&value[2..], 16)?)
    } else {
        Ok(qty_to_int(value.parse::<f64>()?))
    }
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

            // Do we need to run the loop for checking resource stacks?
            let needs_subfilter = search.qty_min > 1 || search.qty_max < 0xfffffe;
            // Do we need to calculate cost? (Moderately expensive)
            let needs_cost =
                search.tolerance < 0.5 || search.cost_min > 1 || search.cost_max < 5392;

            let all_default = search.stack_min <= 1
                && search.stack_max >= 3
                && hash.is_empty()
                && !needs_subfilter
                && !needs_cost;
            for y in y_min..=y_max {
                if all_default {
                    // Raw loop, no filtering
                    for x in x_min..=x_max {
                        print_dim(&Dimension::new(x, y), &mut writer, args.format);
                    }
                } else {
                    // Create stubs just to filter the fields we can see
                    for x in x_min..=x_max {
                        let stub = StubDimension::new(x, y);
                        if !(hash.is_empty() || hash.contains(&stub.name)) {
                            continue;
                        }
                        if stub.stacks.len() < search.stack_min
                            || stub.stacks.len() > search.stack_max
                        {
                            continue;
                        }
                        if needs_subfilter {
                            let fail_pred = |i: usize| {
                                let stack = &stub.stacks[i];
                                stack.qty < search.qty_min || stack.qty > search.qty_max
                            };
                            let len = stub.stacks.len();
                            match search.filter {
                                FilterType::Any => {
                                    if fail_pred(0)
                                        && (len < 3 || fail_pred(2))
                                        && (len < 2 || fail_pred(1))
                                    {
                                        continue;
                                    }
                                }
                                FilterType::All => {
                                    if fail_pred(0)
                                        || (len > 2 && fail_pred(2))
                                        || (len > 1 && fail_pred(1))
                                    {
                                        continue;
                                    }
                                }
                            }
                        }
                        if needs_cost {
                            let cost = stub.cost();
                            let icost: u32 = unsafe { round_ties_even(cost).to_int_unchecked() };
                            if icost < search.cost_min || icost > search.cost_max {
                                continue;
                            }
                            let cost_tol = cost + 0.5;
                            let tol = round_ties_even(cost_tol) - cost_tol;
                            if search.tolerance < tol.abs() {
                                continue;
                            }
                        }
                        print_dim(&Dimension::new(x, y), &mut writer, args.format);
                    }
                }
                progress.inc(x_total);
            }
        }
    }
    writer.flush().expect("Failed to finalize file");
}
