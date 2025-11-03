use clap::Parser;

// cargo run --package cli -- --help
// cargo run --package cli -- --name Alice --count 3
// cargo run --package cli --example simple1 -- --name me --count 3
#[derive(Debug, Parser)]
#[command(about = "A simple Rust CLI tool", version)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello, {}!", args.name);
    }
}
