# scs-telemetry-rs — CLAUDE.md

## Project Vision

Pure-Rust reimplementation of the SCS SDK telemetry system for Euro Truck Simulator 2 (ETS2) and American Truck Simulator (ATS).

**Not a binding.** Goal: rewrite full `scs_sdk` base package in Rust — types, plugin DLL, shared memory writer, and client reader — from scratch.

Reference sources:
- SCS SDK zip: `https://download.eurotrucksimulator2.com/scs_sdk_1_14.zip`
- C++ plugin reference: `https://github.com/RenCloud/scs-sdk-plugin`

---

## Workspace Layout

```
scs-telemetry-rs/
├── Cargo.toml                          # workspace root
├── crates/
│   ├── scs-telemetry-sys/              # raw FFI types from scs_sdk headers
│   ├── scs-telemetry-shared-memory/    # shared memory layout (SCS_Telemetry_Mem)
│   ├── scs-telemetry-sdk/              # safe Rust SDK layer (plugin side)
│   ├── scs-telemetry/                  # main plugin DLL crate (cdylib)
│   └── scs-telemetry-client/           # reader crate for external apps
└── examples/
    └── console-log/                    # minimal example: connect + print telemetry
```

---

## Crate Responsibilities

### `scs-telemetry-sys`
- Raw FFI bindings to SCS SDK C types
- Types: `SCS_SDK_telemetry_version_t`, `SCS_SDK_telemetry_init_params_t`, all channel/config/event enums and structs
- Source: `scssdk_telemetry.h`, `scssdk_telemetry_truck_common_channels.h`, etc. from the SDK zip
- `#[repr(C)]` structs, no_std compatible
- No logic — pure type definitions

### `scs-telemetry-shared-memory`
- Defines `SCS_Telemetry_Mem` — the shared memory layout struct
- Must match exact binary layout used by `scs-telemetry-sdk-plugin` C++ reference
- Contains all truck/job/navigation/controls sub-structs
- Derives `bytemuck::Pod + Zeroable` for zero-copy memory mapping
- Used by both plugin side (writer) and client side (reader)

### `scs-telemetry-sdk`
- Safe Rust wrapper over `scs-telemetry-sys` types
- Provides `TelemetryPlugin` trait for implementing a plugin
- Handles: channel registration, event callbacks, value extraction
- Abstracts over raw C function pointers in init params
- This is the "SDK" that plugin authors use to write Rust plugins

### `scs-telemetry` (main plugin DLL)
- `crate-type = ["cdylib"]`
- Implements `scs_telemetry_init` and `scs_telemetry_shutdown` C exports
- Uses `scs-telemetry-sdk` to register all channels and events
- Writes telemetry data into shared memory via `scs-telemetry-shared-memory`
- Produces `scs_telemetry.dll` — drop into ETS2/ATS plugins folder

### `scs-telemetry-client`
- Reads `SCS_Telemetry_Mem` from shared memory (Windows `OpenFileMapping`)
- Exposes a safe `TelemetryClient` struct with typed accessors
- Intended for external Rust apps (overlays, dashboards, Discord bots, etc.)
- No dependency on plugin-side crates beyond `scs-telemetry-shared-memory`

---

## Key Design Decisions

- **No bindings, full rewrite**: All C header types are reimplemented as Rust `#[repr(C)]` structs. No `bindgen` required (optional tooling only).
- **Shared memory name**: `"Local\\SCSTelemetry"` — must match C++ plugin exactly for interop.
- **`bytemuck` for zero-copy**: `SCS_Telemetry_Mem` must be `Pod`. No padding hacks.
- **`windows-sys` only**: No `winapi`/`windows` crate. Use `windows-sys` for Win32 calls.
- **`no_std` in `scs-telemetry-sys`** where possible. `scs-telemetry-shared-memory` also `no_std`.
- **Plugin DLL target**: `i686-pc-windows-msvc` (ETS2 is 32-bit). Build with cross-compilation or explicit target.

---

## Build Notes

```bash
# Build plugin DLL (32-bit target for ETS2)
cargo build --package scs-telemetry --target i686-pc-windows-msvc --release

# Build client (native arch)
cargo build --package scs-telemetry-client --release

# Run example
cargo run --example console-log
```

Plugin output: `target/i686-pc-windows-msvc/release/scs_telemetry.dll`  
Drop into: `<ETS2 install>/bin/win_x86/plugins/`

---

## Telemetry Channels (from scs-sdk-plugin reference)

All channels from `RenCloud/scs-sdk-plugin` must be covered:

**Truck:**
- Speed, RPM, gear (displayed/input), retarder, cruise control
- Engine on/off, electric on/off, wipers
- Fuel, fuel range, AdBlue
- Odometer, wear (engine/transmission/cabin/chassis/wheels)
- Dashboard lights: hazard, beam, parking, brake, battery, etc.
- Cabin/head/hook position and orientation
- Acceleration (linear/angular, cabin/head)
- Wheel count + per-wheel: susp deflection, velocity, steering, rotation, lift

**Job:**
- Income, deadline, remaining distance/time
- City source/destination, company source/destination
- Cargo: name, id, mass, unit count, unit mass, body type

**Navigation:**
- Estimated time/distance, speed limit

**Trailer (up to 10):**
- Connected state, cargo damage, wear, wheel data
- Position/orientation, hook position

**Controls:**
- Steering, throttle, brake, clutch

---

## Shared Memory Struct Reference

Based on `scs-sdk-plugin/ets2-telemetry-server/SCSSdkClient/scs_sdk_1_00_scania_ets2/...`

Key struct: `scsSDKTelemetry_t` in C++ → `SCS_Telemetry_Mem` in Rust.

Versioning: struct contains `sdkActive: bool` and `time: u32` (game timestamp).

---

## Author

Drysius <daniel.alternight@gmail.com>
