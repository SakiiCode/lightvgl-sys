# lvgl-sys

Rust raw bindings for the LVGL C library.

## Usage

Build requires the following environment variables to be set:

- `DEP_LV_CONFIG_PATH`: Path to the directory containing the `lv_conf.h` header file used for configuration of the LVGL library.

It is better to store it in `.cargo/config.toml` then rust-analyzer will also pick it up.

```toml
[env]
DEP_LV_CONFIG_PATH = { relative = true, value = "." }
```

Alternatively, it can be added to cargo commands:
```sh
DEP_LV_CONFIG_PATH=`pwd` cargo build
```