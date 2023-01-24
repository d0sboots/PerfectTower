//! Utility to make use of the dimension library.

use clap::Parser;
use dimensions::Dimension;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, allow_negative_numbers = true)]
    #[arg(num_args = 2, value_names(["X", "Y"]))]
    /// Show values for a specific dimension, with "x y" coords
    show: Vec<i32>,
}

fn main() {
    let args = Args::parse();
    if let [x, y] = args.show[..] {
        let dim = Dimension::new(x, y);
        println!("Dim {} {}: {}", x, y, dim);
    }
}
