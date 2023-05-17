<p align="center">
    <img src="https://justnatsuki.club/img/logos/installer.png" height="400"/> <!--Update when we have installer logo-->
</p>
<br>

## Just Natsuki Installer

A cross-platform installer for [Just Natsuki](https://github.com/Just-Natsuki-Team/NatsukiModDev).

_dev_:

[![CI](https://github.com/Just-Natsuki-Team/NatsukiModInstaller/actions/workflows/ci.yml/badge.svg?branch=dev)](https://github.com/Just-Natsuki-Team/NatsukiModInstaller/actions/workflows/ci.yml)

_master_:

[![CI](https://github.com/Just-Natsuki-Team/NatsukiModInstaller/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/Just-Natsuki-Team/NatsukiModInstaller/actions/workflows/ci.yml) [![Build](https://github.com/Just-Natsuki-Team/NatsukiModInstaller/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/Just-Natsuki-Team/NatsukiModInstaller/actions/workflows/build.yml)

### Made possible thanks to:

- The [Monika After Story Team](https://github.com/Monika-After-Story/MonikaModDev), for permission to fork and use their [original installer](https://github.com/Monika-After-Story/mas-installer).
- [Booplicate](https://github.com/booplicate), for their kind assistance in developing this fork and their work as the original author of the installer.

**You're all awesome!**

---

## Main Features:

- Cross-platform (Windows, MacOS, Linux)
- Small executables size
- Fast
- Dynamic version fetching
- Detection of DDLC directory

---

## Main Stack:

- Rust 1.62 (can build >= 1.59)
- FLTK 1.3

---

## Structure:

- [`src/static_data.rs`](./src/static_data.rs) - app assets definition (icons, backgrounds, etc), they are backed into the executables
- [`src/app/styles.rs`](./src/app/styles.rs) - app styles (various constants, positions, label text, etc)
- Errors messages are hardcoded in the places they are raised (for now anyway)

---

## Building from Source:

- Building is straightforward using [`cargo`](https://github.com/rust-lang/cargo/)
- - Linux: you might need to install some dependencies (check [this workflow](https://github.com/Monika-After-Story/mas-installer/blob/master/.github/workflows/build.yml) for more details)
- - Windows: for now both `msvc` and `gnu` toolchains are supported, but `msvc` is more preferable
- For all supported features check `Cargo.toml`
- - `include_license` includes `static/license.md` into the binary during building (enabled by default)
- - To disable all default features, use the `--no-default-features` flag in cargo
- - Testing: `cargo run --no-default-features`
- - Building executable: `cargo build --no-default-features`
