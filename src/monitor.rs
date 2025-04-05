use std::thread::sleep;

use derive_more::Sub;
use stopwatch::Stopwatch;
use uom::si::{
    electric_charge::microampere_hour, electric_potential::millivolt, f64::*, power::watt,
    time::millisecond,
};

const GET_COMMAND: &str = "adb shell dumpsys battery";

/// Monitor the battery level and print the power input
pub fn monitor(polling_rate: Time) {
    let mut prev_battery_stats = get_battery_level();
    let mut battery_stats = prev_battery_stats;
    let mut sw = Stopwatch::start_new();
    loop {
        prev_battery_stats = battery_stats;
        battery_stats = get_battery_level();

        let difference = battery_stats.clone() - prev_battery_stats;

        if difference.charge != ElectricCharge::new::<microampere_hour>(0f64) {
            let energy = difference.charge * battery_stats.voltage;
            let time = Time::new::<millisecond>(sw.elapsed_ms() as f64);
            let charge_power = get_power(energy, time);

            println!("Charge power: {:.3} W", charge_power.get::<watt>());

            sw.restart();
        }

        sleep(std::time::Duration::from_millis(
            polling_rate.get::<millisecond>() as u64,
        ));
    }
}

/// Fetch the current battery level using GET_COMMAND and REFRESH_RATE
fn get_battery_level() -> BatteryStats {
    let output = std::process::Command::new("sh")
        .args(["-c", GET_COMMAND])
        .output()
        .expect("Failed to start the adb process");

    if !output.status.success() {
        panic!(
            "Failed to get battery level: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let output = String::from_utf8_lossy(&output.stdout);
    let lines = output.lines();

    let mut battery_stats = BatteryStats {
        charge: ElectricCharge::new::<microampere_hour>(0f64),
        voltage: ElectricPotential::new::<millivolt>(0f64),
    };

    for line in lines {
        if line.contains("Charge counter:") {
            battery_stats.charge = ElectricCharge::new::<microampere_hour>(
                line.split(": ").collect::<Vec<&str>>()[1]
                    .parse::<f64>()
                    .unwrap(),
            );
        }
        if line.contains("voltage:") && !line.contains("Max charging voltage:") {
            battery_stats.voltage = ElectricPotential::new::<millivolt>(
                line.split(": ").collect::<Vec<&str>>()[1]
                    .parse::<f64>()
                    .unwrap(),
            );
        }
    }

    battery_stats
}

#[derive(Clone, Sub)]
struct BatteryStats {
    charge: ElectricCharge,
    voltage: ElectricPotential,
}

/// Get power from energy and time
fn get_power(energy: Energy, time: Time) -> Power {
    energy / time
}
