# i3-plasma

An IPC addon for i3 to integrate it better with plasma.

## Features

* Moves notifications to the top left of the screen
	- handles multiple notifications (placing new notifications below older notifications)

## Installation

See the [releases] page for pre-compiled binaries.

### On Debian/Ubuntu

Download the `.deb` package from the [releases] page and run:

```bash
sudo dpkg -i path/to/package.deb
```

<!-- TODO: AUR repository for Arch
### On Arch

You can install _i3-plasma_ via this AUR: 
-->

<!-- TODO: wait for https://github.com/tmerr/i3ipc-rs/pull/52 then publish to crates
### From source

With Rust's package manager [cargo], you can install _i3-plasma_ via:

```bash
cargo install i3-plasma
```
-->

## Usage

There are many ways to start `i3` and it varies per system. Usually, the simplest way of starting `i3-plasma` is just adding it into your `i3` config file:

```bash
exec i3-plasma
```

[releases]: https://github.com/acheronfail/i3-plasma/releases
[cargo]: https://github.com/rust-lang/cargo
