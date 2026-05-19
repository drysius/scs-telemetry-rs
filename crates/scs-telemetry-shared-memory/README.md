# scs-telemetry-shared-memory

`#[repr(C)]` shared memory layout for the SCS telemetry plugin.

Defines `TelemetryMap` — the struct written by the plugin DLL and read by
client applications through a Windows named shared memory object
(`Local\SCSTelemetry`). Binary-compatible with `scsTelemetryMap_t` from the
[scs-sdk-plugin](https://github.com/RenCloud/scs-sdk-plugin) reference
implementation (PLUGIN_REVISION 12).

Both the plugin ([`scs-telemetry`](../scs-telemetry)) and the client
([`scs-telemetry-client`](../scs-telemetry-client)) depend on this crate —
it is the shared contract between the two sides.

## Usage

```toml
[dependencies]
scs-telemetry-shared-memory = "0.1"
```

```rust
use scs_telemetry_shared_memory::TelemetryMap;

// zero a fresh map
let map = TelemetryMap::new();

// cast raw bytes from shared memory (zero-copy via bytemuck)
use bytemuck::from_bytes;
let map: &TelemetryMap = bytemuck::from_bytes(&raw_bytes);
```

## Key types

| Type | Description |
|---|---|
| `TelemetryMap` | Root struct — entire shared memory region |
| `TruckData` | All truck scalar, placement, and per-wheel fields |
| `TrailerData` | Per-trailer connected state, placement, wear, wheels |
| `JobData` | Cargo, income, source/destination cities and companies |
| `WheelData` / `TrailerWheelData` | Per-wheel suspension, velocity, rotation, lift |

## Constants

- `SHARED_MEMORY_NAME` = `"Local\\SCSTelemetry"`
- `PLUGIN_REVISION` = `12`
- `MAX_WHEEL_COUNT` = `14`
- `MAX_TRAILER_COUNT` = `10`

## License

MIT OR Apache-2.0
