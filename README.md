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

## Optional configuration

You might need to set one or more of these env variables, especially when cross-compiling:
- `LV_COMPILE_ARGS`:  Extra arguments to be passed both to bindgen and to the C compiler
- `LV_SYSROOT`: Path to the bindgen sysroot folder (`--sysroot ...`)
- `LV_EXTRA_C_FILES`: Path to a directory containing additional C source code, for example custom fonts
- `CROSS_COMPILE`: Target triple when cross-compiling
- `LIBCLANG_PATH`: Path to the directory containing *libclang.so* or *libclang.dll*

Setting `BINDGEN_EXTRA_CLANG_ARGS` directly is discouraged as other crates might interfere with it,
for example `esp-idf-sys` might override the value.



## Compatibility table

> [!IMPORTANT]  
> This crate does not follow semantic versioning. The major and minor versions are tied to the LVGL releases.
> Breaking changes can be introduced in minor releases, additional functionality are implemented in patch updates.
> It is recommended to use the [`~` operator](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#tilde-requirements) to prevent auto updating minor versions: 
>```
>lightvgl-sys = "~9.5.5"
>```


| lightvgl-sys | LVGL  |
| ------------ | ----- |
| 9.5.x        | 9.5.0 |
| 9.4.x        | 9.4.0 |
| 9.3.x        | 9.3.0 |
| 9.2.x        | 9.2.2 |

## See also

If looking for safe bindings, check out [lv_bevy_ecs](https://github.com/SakiiCode/lv_bevy_ecs)
