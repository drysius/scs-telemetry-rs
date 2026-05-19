# scs-telemetry

SCS telemetry plugin DLL for Euro Truck Simulator 2 and American Truck Simulator.

Exports `scs_telemetry_init` and `scs_telemetry_shutdown` matching the SCS SDK
C ABI. On load, registers all truck, trailer, job, and navigation channels and
writes live telemetry into a Windows named shared memory object
(`Local\SCSTelemetry`) readable by [`scs-telemetry-client`](../scs-telemetry-client).

## Installation

1. Build the DLL:

```sh
cargo build -p scs-telemetry --target x86_64-pc-windows-msvc --release
```

2. Copy `scs_telemetry.dll` to your game's plugin folder:

```
<ETS2 install path>\bin\win_x64\plugins\scs_telemetry.dll
```

3. Launch the game — the plugin loads automatically.

## Supported telemetry

- **Truck**: speed, RPM, gear, fuel, AdBlue, oils, temperatures, battery,
  all dashboard lights, cruise control, retarder, wear, odometer, navigation,
  all inputs/effective values, per-wheel suspension/velocity/rotation/lift
- **Placement**: world position, cabin offset, head offset, velocities,
  accelerations
- **Job**: cargo damage, delivery info (via configuration event)
- **Trailer**: connected state (slot 0)
- **Events**: frame start (timestamp), paused/started, configuration
  (wheel count), gameplay

## Supported SDK versions

- Telemetry API 1.00
- Telemetry API 1.01

## License

MIT OR Apache-2.0
