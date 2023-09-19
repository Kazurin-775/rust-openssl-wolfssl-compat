# rust-openssl-wolfssl-compat

## Overview

This project is mostly [rust-openssl](https://github.com/sfackler/rust-openssl), but modified to link against wolfSSL. This acts as a compatibility layer, such that other crates that depends on `rust-openssl` should also be able to work with wolfSSL.

The initial motivation of this project is to support OpenWrt's built-in wolfSSL library, such that Rust applications deployed on routers could occupy much less storage space (&lt; 2 MiB), while still maintaining a decent level of security.

## Example usage

In your Rust project that makes use of either `hyper-openssl`, `tokio-openssl` or anything, add the following lines to `Cargo.toml`, so that Cargo will link it against our customized `rust-openssl`:

```toml
[patch.crates-io]
openssl = { git = "https://github.com/Kazurin-775/rust-openssl-wolfssl-compat.git" }
openssl-sys = { git = "https://github.com/Kazurin-775/rust-openssl-wolfssl-compat.git" }
```

(Note: it is recommended that you pin these 2 dependencies to a specific Git commit, in case there were any unwanted changes introduced upstream.)

---

Next, if your `libwolfssl.so` is not globally installed (e.g. when you are cross-compiling for OpenWrt), tell `rust-openssl` its location through environment variables:

```sh
export OPENSSL_INCLUDE_DIR=${path_to_wolfssl_src}/wolfssl
export OPENSSL_LIB_DIR=${path_to_wolfssl_lib_dir}
# Tell openssl-sys to link against libwolfssl.so
export OPENSSL_LIBS=wolfssl
```

(Note: some wolfSSL libraries offered by system package managers are **incomplete**, and thus cannot be used with this project! In order to fix this, you will need to manually re-compile wolfSSL yourself.)

---

Finally, build and test your application (locally):

```sh
cargo build
LD_LIBRARY_PATH=${path_to_wolfssl_lib_dir} cargo run
```

The cross compiling procedure works similarly; simply modify `.cargo/config.toml` and set `--target` as you usually would.
