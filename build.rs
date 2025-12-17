use std::env::{self, VarError};
use std::path::{Path, PathBuf};

use string_literals::{string_arr, string_vec};

static CONFIG_NAME: &str = "DEP_LV_CONFIG_PATH";

fn env(name: &str, msg: &str) -> String {
    match env::var(name) {
        Ok(result) => result,
        Err(VarError::NotPresent) => {
            panic!("{}", msg);
        }
        Err(VarError::NotUnicode(_)) => {
            panic!("{} must be valid UTF-8.", name);
        }
    }
}

fn main() {
    let project_dir = canonicalize(PathBuf::from(env(
        "CARGO_MANIFEST_DIR",
        "Cargo build scripts always have CARGO_MANIFEST_DIR",
    )));
    let vendor = project_dir.join("vendor");

    println!("cargo:rerun-if-env-changed={}", CONFIG_NAME);

    // if use-vendored-config is enabled, autodetect lv_conf.h in the vendor folder
    let mut compiler_args = string_vec!["-DLV_USE_PRIVATE_API=1"];

    // if disabled, define LV_CONF_INCLUDE_SIMPLE=1 and include the config folder
    if !cfg!(feature = "use-vendored-config") {
        let config_path = env(
            CONFIG_NAME,
            "lv_conf.h not found. Set DEP_LV_CONFIG_PATH to its directory.",
        );

        let conf_path = PathBuf::from(config_path);
        if !conf_path.exists() {
            panic!(
                "Directory {} referenced by {} needs to exist",
                conf_path.display(),
                CONFIG_NAME
            );
        }
        if !conf_path.is_dir() {
            panic!("{} needs to be a directory", CONFIG_NAME);
        }
        if !conf_path.join("lv_conf.h").exists() {
            panic!(
                "Directory {} referenced by {} needs to contain a file called lv_conf.h",
                conf_path.display(),
                CONFIG_NAME
            );
        }
        println!(
            "cargo:rerun-if-changed={}",
            conf_path.join("lv_conf.h").display()
        );

        compiler_args.extend(string_arr![
            "-DLV_CONF_INCLUDE_SIMPLE=1",
            "-I",
            conf_path.to_string_lossy().to_string(),
        ]);
    };

    // Set correct target triple for bindgen when cross-compiling
    let target = env::var("CROSS_COMPILE").map_or_else(
        |_| env("TARGET", "Cargo build scripts always have TARGET"),
        |c| c.trim_end_matches('-').to_owned(),
    );
    let host = env("HOST", "Cargo build scripts always have HOST");
    let cross_compile_flags = if target != host {
        string_vec!["-target", &target]
    } else {
        string_vec![]
    };

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_args(
            &compiler_args
                .iter()
                .chain(&cross_compile_flags)
                .collect::<Vec<&String>>(),
        )
        // The input header we would like to generate
        // bindings for.
        .header(vendor.join("lvgl/lvgl.h").to_string_lossy())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Layout tests fail when cross-compiling
        .layout_tests(false)
        // Wrapping unsafe ops is necessary for Rust 2024 edition
        .wrap_unsafe_ops(true)
        // Use ::core for no_std compatibility
        .use_core()
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env("OUT_DIR", "Cargo build scripts always have OUT_DIR"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings.rs!");

    #[cfg(feature = "library")]
    compile_library(compiler_args, vendor);
}

#[cfg(feature = "library")]
fn compile_library(compiler_args: Vec<String>, vendor: PathBuf) {
    let target = env("TARGET", "Cargo build scripts always have TARGET");

    let lvgl_src = vendor.join("lvgl").join("src");

    let mut cfg = cc::Build::new();

    add_c_files(&mut cfg, &lvgl_src);

    // Fix for ESP32
    if target.starts_with("xtensa") {
        cfg.flag("-mlongcalls");
    }

    compiler_args.iter().for_each(|arg| {
        let _ = cfg.flag(arg);
    });

    cfg.compile("lvgl");
}

#[cfg(feature = "library")]
fn add_c_files(build: &mut cc::Build, path: impl AsRef<Path>) {
    for e in path.as_ref().read_dir().unwrap() {
        let e = e.unwrap();
        let path = e.path();
        if e.file_type().unwrap().is_dir() {
            add_c_files(build, e.path());
        } else if path.extension().and_then(|s| s.to_str()) == Some("c") {
            build.file(&path);
        }
    }
}

fn canonicalize(path: impl AsRef<Path>) -> PathBuf {
    let canonicalized = path.as_ref().canonicalize().unwrap();
    let canonicalized = &canonicalized.to_string_lossy();

    PathBuf::from(canonicalized.strip_prefix(r"\\?\").unwrap_or(canonicalized))
}
