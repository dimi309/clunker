extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::string::String;

#[cfg(target_os = "windows")]
fn get_vulkan_sdk() -> String {
    return env!("VULKAN_SDK").to_string();
}

#[cfg(target_os = "linux")]
fn get_vulkan_sdk() -> String {
    return " ".to_string();
}

fn main() {

    // env::var runs at runtime
    // env! runs at compile time
    let vulkan_sdk = get_vulkan_sdk();

    println!("cargo:rustc-link-search=lib");
    println!("cargo:rustc-link-search={}\\Lib", vulkan_sdk);

    println!("cargo:rustc-link-lib=vulkan_helper");
    if cfg!(windows) {
	println!("cargo:rustc-link-lib=vulkan-1");
	println!("cargo:rerun-if-changed=wrapper_win32.h");
    } else {
	println!("cargo:rustc-link-lib=vulkan");
	println!("cargo:rerun-if-changed=wrapper.h");
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I".to_owned() + &vulkan_sdk + r#"\Include"#)
        .blocklist_item("_IMAGE_TLS_DIRECTORY64")
        .blocklist_item("IMAGE_TLS_DIRECTORY64")
        .blocklist_item("PIMAGE_TLS_DIRECTORY64")
        .blocklist_item("IMAGE_TLS_DIRECTORY")
        .blocklist_item("PIMAGE_TLS_DIRECTORY64")
        .blocklist_item("PIMAGE_TLS_DIRECTORY")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");


    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    
}
