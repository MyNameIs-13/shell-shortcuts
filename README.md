# COSMIC Pop Shortcuts

Application for displaying Pop Keyboard Shortcuts.

![](screenshot.png)

## Getting Started

I have no idea what I am doing.
GUI development and Rust are both out of my wheelhouse

## Development

Based on https://github.com/edfloreshz/cosmic-app-template

## Install

To install your COSMIC application, you will need [just](https://github.com/casey/just), if you're on Pop!\_OS, you can install it with the following command:

```sh
sudo apt install just
```

After you install it, you can run the following commands to build and install your application:

```sh
cargo update --precise c751217 cosmic-text  # https://github.com/pop-os/libcosmic/issues/596
just build-release
sudo just install
```

To uninstall simply run

```sh
sudo just uninstall
```

## Publish

To create a `.deb` package you'll need to install `cargo-deb` if you haven't already.
```sh
cargo install cargo-deb
```

Then you can create a `.deb` package for your application with:
```sh
just debpkg
```

With cargo `cargo-deb` your package configuration is in `Cargo.toml` in the `[package.metadata.deb]` section.
Since TOML doesn't really support variables you have to update everything manually (for now).


## Run

Execute the `pop-shortcuts` binary. Assuming the binary has been installed in your path, run it as usual.
```sh
pop-shortcuts
```
