extern crate rustc_version;
use rustc_version::{version, Version};

fn main() {
    if version().unwrap() >= Version::parse("1.25.0").unwrap() {
        println!("cargo:rustc-cfg=has_repr_align");
    }
    if version().unwrap() >= Version::parse("1.26.0").unwrap() {
        println!("cargo:rustc-cfg=has_localkey_try_with");
    }
}
