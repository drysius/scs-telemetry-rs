# scs-telemetry-sys

`#![no_std]` FFI type definitions for the SCS SDK C ABI.

Covers `scssdk.h`, `scssdk_value.h`, `scssdk_telemetry.h`,
`scssdk_telemetry_truck_common_channels.h`,
`scssdk_telemetry_job_common_channels.h`, and the gameplay/configuration
event headers — all hand-written as `#[repr(C)]` Rust structs with no
`bindgen` dependency.

## Usage

This crate is a low-level building block. Most users should depend on
[`scs-telemetry-client`](../scs-telemetry-client) instead.

```toml
[dependencies]
scs-telemetry-sys = "0.1"
```

## Contents

- **`sdk`** — primitive aliases (`ScsU32`, `ScsFloat`, …), math types
  (`ScsFVector`, `ScsDVector`, `ScsEuler`, `ScsFPlacement`, `ScsDPlacement`),
  value union (`ScsValue`), init params, callback typedefs, result codes
- **`channels`** — all channel name string constants
- **`events`** — configuration / gameplay event ID and attribute name constants

## License

MIT OR Apache-2.0
