
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {

    // Binding the CString to a variable so that it does not get deallocated
    
    let nameStr = CString::new("Hello Rust").expect("CString::new failed");
    let namep = nameStr.as_ptr();

    const exts: *mut *const std::os::raw::c_char = std::ptr::null_mut();

    unsafe {

        // Using the vulkan helper
        let ret = vh_create_instance(namep, exts, 0);
        if ret > 0 {
            println!("Vulkan worked!")
        }
        vh_shutdown();
    }

}
