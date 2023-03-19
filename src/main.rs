use std::io;
use crate::monitor::monitor;

mod monitor;

pub const GET_COMMAND: &str = "adb shell dumpsys battery";
pub const REFRESH_RATE: f64 = 0.1;

fn main() -> Result<(), io::Error> {
    monitor();

    Ok(())
}
