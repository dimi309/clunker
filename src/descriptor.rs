// Don't go crazy with warnings about all the stuff imported from C
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_imports)]

use super::renderer::*;

pub fn create_ubo_buffer<T>(ubo: &T) -> (VkBuffer, VkDeviceMemory) {
    let mut buffer: VkBuffer = std::ptr::null_mut();
    let mut memory: VkDeviceMemory = std::ptr::null_mut();

    unsafe {
        vh_create_buffer(
            &mut buffer,
            VkBufferUsageFlagBits_VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT as u32,
            std::mem::size_of::<T>() as u32,
            &mut memory,
            VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32
                | VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32,
        );
    }
    
    update_ubo_buffer(ubo, memory); 

    (buffer, memory)
}

pub fn update_ubo_buffer<T>(ubo: &T, memory: VkDeviceMemory) {

    let mut mapped_data: *mut ::std::os::raw::c_void = std::ptr::null_mut();
    unsafe {
        let r = vkMapMemory(
            vh_logical_device,
            memory,
            0,
            std::mem::size_of::<T>() as u64,
            0,
            &mut mapped_data,
        );

        assert!(r == 0);
    }

    let ubo_as_ptr = ubo as *const T;

    unsafe {
        std::ptr::copy_nonoverlapping(
            ubo_as_ptr as *const u8,
            mapped_data as *mut u8,
            std::mem::size_of::<T>(),
        );

        vkUnmapMemory(vh_logical_device, memory);
    }

}
