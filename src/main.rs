pub mod monitor;

use clap::{command, value_parser, Arg};
use uom::si::{f64::*, time::second};

use crate::monitor::monitor;

fn main() {
    let matches = command!()
        .arg(
            Arg::new("refresh-rate")
                .short('r')
                .long("refresh-rate")
                .default_value("0.1")
                .value_parser(value_parser!(f64))
                .help(
                    "Sets polling rate in seconds (Doesn't change actual Android battery stats \
                     refresh rate)",
                ),
        )
        .get_matches();

    let polling_rate = matches.get_one::<f64>("refresh-rate").unwrap_or(&0.1f64);

    let polling_rate = Time::new::<second>(*polling_rate);
    monitor(polling_rate);
}
