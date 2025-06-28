#![no_std]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "raw-bindings")]
pub fn _bindgen_raw_src() -> &'static str {
    include_str!(concat!(env!("OUT_DIR"), "/bindings.rs"))
}