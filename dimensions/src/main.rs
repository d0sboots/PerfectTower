use clap::Parser;
use dimensions::Dimension;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    xcoord: i32,
    ycoord: i32,
}

fn main() {
    let args = Args::parse();
    let dim = Dimension::new(args.xcoord, args.ycoord);
    println!("Dim {} {}: {}", args.xcoord, args.ycoord, dim.name());
}
