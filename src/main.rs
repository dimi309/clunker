
// Don't crazy with warnings about all the stuff imported from C
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {

    // Binding the CString to a variable to avoid it being deallocating it, which
    // would have been the case if we just assigned it to a pointer with .as_ptr()
    // after instanciation.
    
    let nameStr = CString::new("Hello Rust").expect("CString::new failed");
 
    let namep = nameStr.as_ptr();

    // Null pointer
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
