# lightvgl-sys

Rust raw bindings for the LVGL C library.

## Usage

Build requires the following environment variables to be set:

- `DEP_LV_CONFIG_PATH`: Path to the directory containing the `lv_conf.h` header file used for configuration of the LVGL library.

It is easier to store them in `.cargo/config.toml` then rust-analyzer will also pick them up.

```toml
[env]
DEP_LV_CONFIG_PATH = { relative = true, value = "." }
```

Alternatively, it can be added before cargo commands:

```sh
DEP_LV_CONFIG_PATH=`pwd` cargo build
```

## See also

If looking for safe bindings, check out [lv_bevy_ecs](https://github.com/SakiiCode/lv_bevy_ecs)
