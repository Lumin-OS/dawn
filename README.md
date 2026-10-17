# Dawn

The LuminOS installer: a Rust/Slint GUI and a privileged backend, with
runtime branding. The frontend runs as the live user; only the install
backend goes through pkexec after disk selection and confirmation.

## Development

Run the mock GUI without switching the host session's keyboard:

```sh
env -u HYPRLAND_INSTANCE_SIGNATURE cargo run -p frontend -- --mock-backend
```

Preview a different branding folder without recompiling:

```sh
DAWN_BRANDING_DIR=/path/to/branding env -u HYPRLAND_INSTANCE_SIGNATURE \
  cargo run -p frontend -- --mock-backend
```

See [branding/README.md](branding/README.md) for images, slides and
captions, and [po/README.md](po/README.md) for gettext. The distribution's
installer configuration is `/etc/dawn/installer.toml`; `DAWN_CONFIG`
overrides the GUI and unprivileged probes during development. The root
backend always uses system configuration.

## Checks

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
cargo run -p backend -- --dry-run tests/plans/erase-online.json
```

`frontend/tests/branding.rs` checks the M5 branding acceptance point:
replacing a folder changes the same compiled UI, including slide rotation.
Installer integration tests use disposable VMs and a local package mirror.
Never run a real install or Secure Boot enrollment on a development host.

## ISO integration

[LuminOS](https://github.com/Lumin-OS/LuminOS) owns the archiso profile,
`luminos-live` and the M5 real-ISO boot smoke test.
[luminos-packages](https://github.com/Lumin-OS/luminos-packages) owns the
`luminos-dawn` PKGBUILD; it installs both binaries, polkit files, default
configuration, launcher and `/usr/share/dawn/branding/luminos`.

The default desktop loads `keyboard.lua`, and greetd reads
`/etc/greetd/luminos.toml`. Offline installs remove `luminos-live`, including
its live-user autologin and boot-only services. All matching packages must
be built and published before distributing an ISO. M5's boot check must
pass before claiming the real ISO boots into Dawn; nightly installation
checks of the real ISO belong to M6.
