# scs-telemetry-rs

Pure-Rust reimplementation of the SCS telemetry plugin for **Euro Truck Simulator 2** and **American Truck Simulator**.

This is **not** a binding to the C++ plugin — it is a ground-up Rust rewrite of the full telemetry stack: FFI types, shared memory layout, plugin DLL, and client reader.

## Workspace crates

| Crate | Description |
|---|---|
| [`scs-telemetry-sys`](crates/scs-telemetry-sys) | `#![no_std]` FFI types matching the SCS SDK C ABI |
| [`scs-telemetry-shared-memory`](crates/scs-telemetry-shared-memory) | `#[repr(C)]` shared memory layout (`TelemetryMap`) |
| [`scs-telemetry`](crates/scs-telemetry) | Plugin DLL — drop into the game's `plugins/` folder |
| [`scs-telemetry-client`](crates/scs-telemetry-client) | Client library — read live telemetry from any Rust app |

## Quick start

### Plugin (game side)

```toml
# not needed — just drop the DLL into the game folder
```

Build the plugin DLL and copy it to the game:

```sh
cargo build -p scs-telemetry --target x86_64-pc-windows-msvc --release
copy target\x86_64-pc-windows-msvc\release\scs_telemetry.dll "<ETS2 path>\bin\win_x64\plugins\"
```

### Client (your app)

```toml
[dependencies]
scs-telemetry-client = "0.1"
```

```rust
use scs_telemetry_client::TelemetryReader;

fn main() {
    let reader = TelemetryReader::open().expect("plugin not running");
    loop {
        let data = reader.read();
        println!("speed: {:.1} km/h", data.truck.speed * 3.6);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
```

## Building

```sh
# Check entire workspace
cargo check --workspace

# Build plugin DLL (Windows)
cargo build -p scs-telemetry --target x86_64-pc-windows-msvc --release

# Run the console example (requires plugin running in-game)
cargo run --example console-log
```

## Reference

- SCS SDK: <https://download.eurotrucksimulator2.com/scs_sdk_1_14.zip>
- C++ reference plugin: <https://github.com/RenCloud/scs-sdk-plugin>

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
