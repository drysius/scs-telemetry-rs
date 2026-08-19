use scs_telemetry_client::TelemetryReader;
use std::thread;
use std::time::Duration;

fn main() {
    let reader = match TelemetryReader::open() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to connect: {e}");
            std::process::exit(1);
        }
    };

    println!("Connected to SCS telemetry. Press Ctrl+C to quit.\n");

    loop {
        let map = reader.read();
        if map.sdk_active == 0 {
            println!("[waiting for game...]");
        } else {
            let t = &map.truck;
            let speed_kmh = t.speed * 3.6;
            println!(
                "speed={speed_kmh:6.1} km/h  rpm={:6.0}  gear={:+3}  fuel={:.1} L  paused={}",
                t.engine_rpm,
                t.displayed_gear,
                t.fuel,
                map.paused,
            );
        }

        thread::sleep(Duration::from_secs(1));
    }
}
