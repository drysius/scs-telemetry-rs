# Changelog

All notable changes to this project are documented here. The format is based
on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- New `scs-telemetry-sdk` crate: safe Rust SDK for writing SCS plugins.
  Provides `Value`, `EventInfo`, `AttributesIter`, `InitParams`, the
  `Plugin` trait, and a `define_plugin!` macro that wires the C ABI
  entry points.
- Full SDK 1.14 multi-trailer coverage in `scs-telemetry`: 10 trailer
  slots × all per-trailer channels (scalars + placements + per-wheel
  arrays), dynamically named at registration time.

### Changed
- `scs-telemetry-shared-memory` `lib.rs` split into themed modules
  (`consts`, `common`, `truck`, `trailer`, `config`, `gameplay`,
  `special`, `map`, `util`). Public API unchanged via re-exports.
- README + CLAUDE.md updated to reference SCS Telemetry SDK 1.14 and
  the new SDK crate.

## [0.1.0] - 2026-05-14

### Added
- Initial workspace with four crates:
  - `scs-telemetry-sys` — hand-written `#![no_std]` FFI for the SCS SDK C ABI
    (`scssdk*.h`), including channel / config / gameplay name constants
    and compile-time layout assertions.
  - `scs-telemetry-shared-memory` — `#[repr(C)]` `TelemetryMap` (21 600 bytes,
    `bytemuck::Pod + Zeroable`) byte-compatible with the upstream
    `scsTelemetryMap_t` (`PLUGIN_REVISION = 12`).
  - `scs-telemetry` — Windows `cdylib` exporting `scs_telemetry_init`
    and `scs_telemetry_shutdown`. Supports telemetry API 1.00 and 1.01.
    Registers common, truck (scalars + per-wheel arrays), job, and
    placement channels; handles `configuration` and `gameplay` events
    (job delivered/cancelled, fined, tollgate, ferry, train).
  - `scs-telemetry-client` — `TelemetryReader` that opens `Local\SCSTelemetry`
    read-only and returns volatile snapshots; `dump` example.
- Dual MIT / Apache-2.0 license.
- `CLAUDE.md` contributor / agent guide.

[Unreleased]: https://github.com/drysius/scs-telemetry-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/drysius/scs-telemetry-rs/releases/tag/v0.1.0
