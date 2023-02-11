//! Utility to make use of the dimension library.

use ahash::AHashSet;
use clap::{Args, Parser, Subcommand, ValueEnum};
use dimensions::stub::Dimension as StubDimension;
use dimensions::{
    qty_to_int, round_ties_even, Dimension, DimensionalResource, ResourceFilterOpts, ResourceName,
};
use dpc_pariter::IteratorExt;
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
    /// Number of threads to use when searching. Default is based on the number of cores.
    threads: Option<usize>,

    #[arg(short, long)]
    #[arg(default_value = "")]
    /// Regex that the dimension name must match. Case-insensitive.
    /// Implicitly includes starting/ending ^$
    name: String,

    #[arg(long, value_name = "MIN")]
    #[arg(default_value = "1")]
    /// Minimum number of resource stacks (shown as "Resources" in-game) per dimension
    stack_min: usize,

    #[arg(long, value_name = "MAX")]
    #[arg(default_value = "3")]
    /// Maximum number of resource stacks (shown as "Resources" in-game) per dimension, inclusive
    stack_max: usize,

    #[arg(long)]
    #[arg(default_value = "any")]
    /// How to filter resource stacks. "all" means all stacks must meet the critera, "any" means
    /// at least one must.
    filter: FilterType,

    #[arg(long, value_parser = parse_qty, value_name = "MIN")]
    #[arg(default_value = "0.001")]
    /// Minimum res/sec for resource stacks. Can also be specified in hex from 0x000000 to 0xffffff.
    qty_min: u32,

    #[arg(long, value_parser = parse_qty, value_name = "MAX")]
    #[arg(default_value = "8.5")]
    /// Maximum res/sec for resource stacks. Can also be specified in hex from 0x000000 to 0xffffff. Inclusive.
    qty_max: u32,

    #[arg(long, default_value = "0.5")]
    /// Search for dimensions with a cost within <tolerance> of x.5. This stress-tests the
    /// generation algorithm, and is of little use otherwise.
    tolerance: f64,

    #[arg(long, value_name = "MIN", default_value = "1")]
    /// Minimum cost for dimensions.
    cost_min: u32,

    #[arg(long, value_name = "MAX", default_value = "5392")]
    /// Maximum cost for dimensions. Inclusive.
    cost_max: u32,

    #[arg(long, default_value = "")]
    /// Regex that the resource name must match. This matches the full name, including flavor text
    /// bits like "Ore" or "Droplets of". Case-insensitive. Implicitly includes starting/ending ^$
    resource_name: String,

    #[arg(long, value_name = "MIN", default_value = "1")]
    /// Minimum number of properties on a resource.
    properties_min: usize,

    #[arg(long, value_name = "MAX", default_value = "5")]
    /// Maximum number of properties on a resource. Set this to 1 to find "pures". Inclusive.
    properties_max: usize,

    #[arg(long, value_name = "MIN", default_value = "1")]
    /// Minimum value that *some* property much achieve to be a valid resource.
    anyprop_min: u8,

    #[arg(long, value_name = "MAX", default_value = "100")]
    /// Maximum value that *some* property much achieve to be a valid resource. Inclusive.
    anyprop_max: u8,

    #[arg(long, value_name = "MIN", default_value = "1")]
    /// Minimum value that *all* properties much achieve to be a valid resource.
    allprop_min: u8,

    #[arg(long, value_name = "MAX", default_value = "100")]
    /// Maximum value that *all* properties much achieve to be a valid resource. Inclusive.
    allprop_max: u8,

    #[arg(long, value_name = "MIN", default_value = "1")]
    /// Minimum sum of all property values. Note that as there are more properties, their maximum
    /// decreases, so that the sum stays roughly in the same range.
    sumprop_min: u8,

    #[arg(long, value_name = "MAX", default_value = "105")]
    /// Minimum sum of all property values. Note that as there are more properties, their maximum
    /// decreases, so that the sum stays roughly in the same range.
    sumprop_max: u8,

    #[arg(long, value_name = "MIN", default_value = "0.2")]
    /// Minimum of the inverse sum of reciprocals of property values. This is equal to the harmonic
    /// mean divided by the number of properties. This is a good measure of the "value" of a
    /// resource, with higher being linearly better.
    invsum_min: f64,

    #[arg(long, value_name = "MAX", default_value = "100")]
    /// Maximum of the inverse sum of reciprocals of property values. This is equal to the harmonic
    /// mean divided by the number of properties. This is a good measure of the "value" of a
    /// resource, with higher being linearly better.
    invsum_max: f64,
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
        Some(ref path) => {
            file = File::create(path).unwrap();
            &mut file
        }
    });
    match args.command {
        Command::Show(Show { show }) => {
            if let [x, y] = show[..] {
                let dim = Dimension::new(x, y);
                print_dim(&dim, &mut writer, args.format);
            } else {
                panic!("No args in {show:?}");
            }
        }
        Command::Search(search) => {
            let mut name_hash = AHashSet::<u32>::with_capacity(512);
            if !search.name.is_empty() {
                let re =
                    Regex::new(&format!("(?i-u)^{}$", search.name)).expect("Failed to parse name");
                eprintln!("Finding matching names...");
                let progress = make_progress(1 << 24);
                let mut i = 0u32;
                while i < (1 << 24) {
                    progress.set_position(i.into());
                    for _ in 0..(1 << 17) {
                        ResourceName::filter(&re, i, &mut name_hash);
                        i += 1
                    }
                }
                progress.finish_and_clear();
                eprintln!("Found {} matching names.\n", name_hash.len());
                if name_hash.is_empty() {
                    return;
                }
            }
            let needs_resource = !search.resource_name.is_empty()
                || search.properties_min > 1
                || search.properties_max < 5
                || search.anyprop_min > 1
                || search.anyprop_max < 100
                || search.allprop_min > 1
                || search.allprop_max < 100
                || search.sumprop_min > 1
                || search.sumprop_max < 105
                || search.invsum_min > 0.2
                || search.invsum_max < 100.0;
            let mut resource_hash = AHashSet::<u32>::with_capacity(512);
            if needs_resource {
                let opts = ResourceFilterOpts {
                    name: if search.resource_name.is_empty() {
                        None
                    } else {
                        Some(
                            Regex::new(&format!("(?i-u)^{}$", search.resource_name))
                                .expect("Failed to parse resource name"),
                        )
                    },
                    properties_min: search.properties_min,
                    properties_max: search.properties_max,
                    anyprop_min: search.anyprop_min,
                    anyprop_max: search.anyprop_max,
                    allprop_min: search.allprop_min,
                    allprop_max: search.allprop_max,
                    sumprop_min: search.sumprop_min,
                    sumprop_max: search.sumprop_max,
                    invsum_min: search.invsum_min,
                    invsum_max: search.invsum_max,
                };
                eprintln!("Finding matching resources...");
                let progress = make_progress(1 << 24);
                let mut i = 0u32;
                while i < (1 << 24) {
                    progress.set_position(i.into());
                    for _ in 0..(1 << 17) {
                        DimensionalResource::filter(&opts, i, &mut resource_hash);
                        i += 1
                    }
                }
                progress.finish_and_clear();
                eprintln!("Found {} matching resources.\n", resource_hash.len());
                if resource_hash.is_empty() {
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
            let needs_subfilter =
                search.qty_min > 1 || search.qty_max < 0xfffffe || !resource_hash.is_empty();
            // Do we need to calculate cost? (Moderately expensive)
            let needs_cost =
                search.tolerance < 0.5 || search.cost_min > 1 || search.cost_max < 5392;

            let all_default = search.stack_min <= 1
                && search.stack_max >= 3
                && name_hash.is_empty()
                && !needs_subfilter
                && !needs_cost;
            if all_default {
                // Raw loop, no filtering
                for y in y_min..=y_max {
                    for x in x_min..=x_max {
                        print_dim(&Dimension::new(x, y), &mut writer, args.format);
                    }
                    progress.inc(x_total);
                }
            } else {
                // Create stubs just to filter the fields we can see
                let process_fn = move |y| {
                    let mut found = Vec::<StubDimension>::new();
                    for x in x_min..=x_max {
                        let stub = StubDimension::new(x, y);
                        if !(name_hash.is_empty() || name_hash.contains(&stub.name)) {
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
                                if stack.qty < search.qty_min || stack.qty > search.qty_max {
                                    return true;
                                }
                                !(resource_hash.is_empty() || resource_hash.contains(&stack.seed))
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
                        found.push(stub);
                    }
                    found
                };
                (y_min..=y_max)
                    .parallel_map_custom(
                        |builder| match search.threads {
                            None => builder,
                            Some(x) => builder.threads(x),
                        },
                        process_fn,
                    )
                    .for_each(|arr| {
                        progress.inc(x_total);
                        // This part is single-threaded, in original order.
                        for stub in &arr {
                            print_dim(&Dimension::new(stub.x, stub.y), &mut writer, args.format);
                        }
                        if args.out == None {
                            // When writing to stdout, flush after every chunk, so we see results faster.
                            // We assume results are uncommon, so this won't be a bottleneck.
                            progress.suspend(|| {
                                writer.flush().unwrap();
                            });
                        }
                    });
            }
        }
    }
    writer.flush().expect("Failed to finalize file");
}
