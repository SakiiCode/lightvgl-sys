use std::env;
use std::path::{Path, PathBuf};

static CONFIG_NAME: &str = "DEP_LV_CONFIG_PATH";

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    //println!("cargo:rustc-link-search=/vendor/lvgl");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=SDL2");
    //println!("cargo::rustc-flags=-mlongcalls");

    let project_dir = canonicalize(PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()));
    let vendor = project_dir.join("vendor");

    // let lv_config_dir = env::var(CONFIG_NAME)
    //     .ok()
    println!("cargo:rerun-if-env-changed={}", CONFIG_NAME);
    let lv_config_dir = Some(env::var(CONFIG_NAME).unwrap())
        .map(PathBuf::from)
        .map(|conf_path| {
            if !conf_path.exists() {
                panic!(
                    "Directory {} referenced by {} needs to exist",
                    conf_path.to_string_lossy(),
                    CONFIG_NAME
                );
            }
            if !conf_path.is_dir() {
                panic!("{} needs to be a directory", CONFIG_NAME);
            }
            if !conf_path.join("lv_conf.h").exists() {
                panic!(
                    "Directory {} referenced by {} needs to contain a file called lv_conf.h",
                    conf_path.to_string_lossy(),
                    CONFIG_NAME
                );
            }
            println!(
                "cargo:rerun-if-changed={}",
                conf_path.join("lv_conf.h").to_str().unwrap()
            );
            conf_path
        });

    /*let mut headers = Vec::new();

    if let Some(lv_config_dir) = &lv_config_dir {
        headers.push(lv_config_dir.join("lv_conf.h"));
    }
    headers.push();*/

    let mut compiler_args = Vec::new();
    if let Some(path) = &lv_config_dir {
        compiler_args = vec!["-DLV_CONF_INCLUDE_SIMPLE=1", "-I", path.to_str().unwrap()];
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_args(&compiler_args)
        // The input header we would like to generate
        // bindings for.
        .header(vendor.join("lvgl/lvgl.h").to_str().unwrap())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Layout tests fail when cross-compiling
        .layout_tests(false)
        .wrap_unsafe_ops(true)
        .use_core()
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    #[cfg(feature = "library")]
    compile_library(compiler_args, vendor);
}

#[cfg(feature = "library")]
fn compile_library(compiler_args: Vec<&str>, vendor: PathBuf) {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");

    let lvgl_src = vendor.join("lvgl").join("src");

    let mut cfg = cc::Build::new();

    add_c_files(&mut cfg, &lvgl_src);

    // #cfg(not(target)) does not work here
    if !target.starts_with("x86_64") {
        cfg.flag("-mlongcalls");
    }

    /*if let Some(lv_config_dir) = lv_config_dir {
        //cfg.define("LV_CONF_INCLUDE_SIMPLE", Some("1"));
        //cfg.include(lv_config_dir);
        cfg.flag(flag)
    }*/
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
    let canonicalized = &*canonicalized.to_string_lossy();

    PathBuf::from(canonicalized.strip_prefix(r"\\?\").unwrap_or(canonicalized))
}
