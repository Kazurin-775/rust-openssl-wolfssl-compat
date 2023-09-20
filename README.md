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

## Sidenotes for OpenWrt users

Basically, if you are building wolfSSL applications that run on OpenWrt devices, you will need the following things:

- `/usr/lib/libwolfssl.so.*` from your OpenWrt device
- The corresponding wolfSSL headers (could be obtained from its [GitHub repo](https://github.com/wolfSSL/wolfssl), no compilation needed)

However, there are still some additional steps required to actually make things work. In order to save space on resource-limited devices, OpenWrt **completely removes all sections** (including those containing symbol information) from all ELF files, including `libwolfssl.so.*`. This makes compilers **unable to link against them** (even if their paths are specified properly), resulting in "undefined symbols" errors upon linkage.

The easiest way to work around this issue is to manually generate a "shim" for `libwolfssl.so`, which contains all the symbol information needed by the compiler, but leaves all functions unimplemented. The C source code of the shim looks like the following:

```c
// ...
void wolfSSL_BIO_clear_flags() {}
void wolfSSL_BIO_clear_retry_flags() {}
void wolfSSL_BIO_ctrl() {}
void wolfSSL_BIO_ctrl_pending() {}
void wolfSSL_BIO_ctrl_reset_read_request() {}
void wolfSSL_BIO_do_accept() {}
void wolfSSL_BIO_do_connect() {}
// ...
```

---

We provide a script to automate the process of generating a shim. When you have `libwolfssl.so.*` from your device, simply run the following commands to generate the shim file:

```sh
# List exports using `nm`, then use the extracted information to generate the shim's source code
nm -gD libwolfssl.so.* | ./wolfssl-scripts/generate-shim.py > libwolfssl-shim.c

# Build the shim (assuming a mipsel-softfloat target)
mipsel-linux-muslsf-gcc -shared libwolfssl-shim.c -o libwolfssl-shim.so
```

Then, replace `libwolfssl.so.*` with `libwolfssl-shim.so` (on your PC, not your OpenWrt device!), and you are ready to build! (Be careful not to upload `libwolfssl-shim.so` to your device, since it doesn't provide any actual functionality, its only function is to trick compilers into linking against the "real" one.)

---

Finally, in order to link against a shared library with non-standard names (such as `libwolfssl.so.5.6.3.ee39414e`), you may need to manually tweak the linker arguments (instead of relying on `openssl-sys`'s automatic searching), such as:

```toml
[target.mipsel-unknown-linux-musl]
rustflags = ["-C", "link-args=/path/to/libwolfssl.so.5.6.3.ee39414e"]
```
