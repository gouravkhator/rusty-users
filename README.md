# Rusty Users - Rust Simple GUI Demo showing Random Users' Data in Rust

[![Apache](https://img.shields.io/badge/license-Apache-blue.svg)](./LICENSE.md)

This repo uses Random Data API, to fetch the random users' data and then showcases that in the GUI native application.

## Commands

### Prerequisites

To run/build this Rust project, you have to install development package of OpenSSL in your Linux distro.

This is because, this project indirectly depends on `openssl` Rust crate, which requires the development package of OpenSSL to be installed natively.

```sh
# On Debian and Ubuntu
sudo apt install pkg-config libssl-dev

# On Arch Linux or its derivates
sudo pacman -S openssl

# On Fedora or its derivates
sudo dnf install openssl-devel
```

### Run/Build/Debug Commands

- Run the project (unoptimized build, just for development purposes):

  ```sh
  cargo run
  ```

- Build the project (for an optimized build):

  ```sh
  cargo build
  ```

## todo!()

- [ ] Style various components of this GUI App
- [ ] Add dark mode toggler in appropriate position
- [ ] Build this app for WASM

## credits!()

- Best playlist for motivating me and guiding me, on how to look through Rust docs and solve the issues yourself, is rightly pasted [here](https://youtube.com/playlist?list=PLfyJcJbPAedRqjVaOd-P8wp_Wy9RIN7Oq).
