# i3-plasma-integration

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

### On Arch

Once https://github.com/tmerr/i3ipc-rs/pull/52 is resolved and a new version of `i3` is published, then the [PKGBUILD] will be added to AUR. Until then, you can use the [PKGBUILD] contained in this repository.

<!-- TODO: post to https://www.reddit.com/r/i3wm/comments/bw1yfs/kde_notifications_appearing_in_the_centre_of/ ? -->

<!-- TODO: wait for https://github.com/tmerr/i3ipc-rs/pull/52 then publish to crates
### From source

With Rust's package manager [cargo], you can install _i3-plasma-integration_ via:

```bash
cargo install i3-plasma-integration
```
-->

## Usage

There are many ways to start `i3` and it varies per system. Usually, the simplest way of starting `i3-plasma-integration` is just adding it into your `i3` config file:

```bash
exec i3-plasma-integration
```

[releases]: https://github.com/acheronfail/i3-plasma-integration/releases
[cargo]: https://github.com/rust-lang/cargo
[PKGBUILD]: https://github.com/acheronfail/i3-plasma-integration/blob/master/resources/PKGBUILD
