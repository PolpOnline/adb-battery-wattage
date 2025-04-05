pub mod monitor;

use clap::Parser;
use uom::si::{f64::*, time::second};

use crate::monitor::monitor;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, default_value_t = 0.1)]
    /// Sets polling rate in seconds (Doesn't change actual Android battery
    /// stats refresh rate)
    refresh_rate: f64,
}

fn main() {
    let args = Args::parse();

    let polling_rate = Time::new::<second>(args.refresh_rate);
    monitor(polling_rate);
}
