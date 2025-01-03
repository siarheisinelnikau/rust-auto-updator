# Rust Auto Updator 0.1.0

Initial reusable updater for portable Windows Rust applications. The library is MIT licensed and consists of a UI-independent core and an optional egui 0.37 panel.

## Added

- Stable GitHub release discovery with semantic version comparison and repository-bound asset selection.
- Background checks, throttled progress, cancellation, startup opt-out support, and explicit download requests.
- SHA-256, executable size, PE structure, architecture and optional GUI-subsystem verification before replacement.
- Portable executable replacement after the host exits, restart, rollback after launch failure, and optional healthy-startup acknowledgement.
- Cleanup limited to known staging files; application settings and user data remain with the host.
- Explicit policies for historical filenames, per-asset checksums and GitHub asset digests.
- Renderer-independent controller and an optional egui panel that respects the host theme and wake scheduling.

## Integration

Requires Rust 1.95 or newer; local verification uses Rust 1.96.1 on Windows x64. This is a source library, with no application EXE or installer of its own. The supported installer target is a writable portable Windows executable; other platforms do not have a native installer.

Build both this workspace and the consuming application from sibling directories when using the current path dependencies. Applications must route the documented update-helper protocol before creating their UI and finish pending mutations before handing off an installation.

The native smoke example only replaces isolated fixtures. No user's installed application is updated during validation. Exact minimum-toolchain, x86/ARM64 installation, GPU/compositor latency and real-account network update installation are outside the completed local verification.
