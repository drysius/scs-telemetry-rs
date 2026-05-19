# scs-telemetry-client

Client library for reading live SCS telemetry from any Rust application.

Opens the shared memory written by the [`scs-telemetry`](../scs-telemetry)
plugin DLL and provides zero-copy volatile snapshots of the full
[`TelemetryMap`](../scs-telemetry-shared-memory).

## Usage

```toml
[dependencies]
scs-telemetry-client = "0.1"
```

```rust
use scs_telemetry_client::TelemetryReader;

fn main() {
    let reader = match TelemetryReader::open() {
        Ok(r) => r,
        Err(e) => { eprintln!("not connected: {e}"); return; }
    };

    loop {
        let map = reader.read();
        if map.sdk_active != 0 {
            let t = &map.truck;
            println!(
                "speed={:.1} km/h  gear={:+}  fuel={:.1} L",
                t.speed * 3.6,
                t.displayed_gear,
                t.fuel,
            );
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
```

## API

```rust
// Open the shared memory (returns Err if plugin is not running)
TelemetryReader::open() -> Result<TelemetryReader, Error>

// Read a snapshot (volatile read, no locking needed for read-only use)
reader.read() -> TelemetryMap

// Quick active check
reader.is_active() -> bool
```

## Platform

Windows only — shared memory is backed by `OpenFileMappingA`.
Returns `Err(Error::Unsupported)` on non-Windows platforms at compile time.

## License

MIT OR Apache-2.0
