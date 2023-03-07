extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    let vulkan_sdk = env::var("VULKAN_SDK").expect("vulkan sdk environment variable not found");

    println!("cargo:rustc-link-search=lib");
    println!("cargo:rustc-link-search={}{}", vulkan_sdk, r#"\Lib"#);

    println!("cargo:rustc-link-lib=vulkan_helper");
    println!("cargo:rustc-link-lib=vulkan-1");

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I".to_owned() + &vulkan_sdk + r#"\Include"#)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    
}
