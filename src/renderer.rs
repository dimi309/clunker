// Don't go crazy with warnings about all the stuff imported from C
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_imports)]

#[cfg(target_os = "linux")]
use winit::platform::x11::WindowExtX11;

#[cfg(target_os = "windows")]
use winit::platform::windows::WindowExtWindows;

#[cfg(target_os = "macos")]
use winit::platform::macos::WindowExtMacOS;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CString;
use std::ptr::addr_of;

const NUM_FRAMES_IN_FLIGHT: usize = 3;

static mut binding_desc: VkVertexInputBindingDescription = VkVertexInputBindingDescription {
    binding: 0,
    stride: 4u32 * (std::mem::size_of::<f32>() as u32),
    inputRate: 0,
};

static mut attrib_desc: VkVertexInputAttributeDescription = VkVertexInputAttributeDescription {
    binding: 0,
    location: 0,
    format: VkFormat_VK_FORMAT_R32G32B32A32_SFLOAT,
    offset: 0,
};

static mut command_buffer: [VkCommandBuffer; NUM_FRAMES_IN_FLIGHT] = [std::ptr::null_mut(); 3];

unsafe extern "C" fn set_input_state_callback(
    inputStateCreateInfo: *mut VkPipelineVertexInputStateCreateInfo,
) -> i32 {
    println!("Input state callback called.");

    (*inputStateCreateInfo).vertexBindingDescriptionCount = 1;
    (*inputStateCreateInfo).vertexAttributeDescriptionCount = 1;
    (*inputStateCreateInfo).pVertexBindingDescriptions = addr_of!(binding_desc);
    (*inputStateCreateInfo).pVertexAttributeDescriptions = addr_of!(attrib_desc);
    1
}

unsafe extern "C" fn set_pipeline_layout_callback(
    pipelineLayoutCreateInfo: *mut VkPipelineLayoutCreateInfo,
) -> i32 {
    println!("Pipeline layout callback called.");
    (*pipelineLayoutCreateInfo).pSetLayouts = std::ptr::null_mut();
    (*pipelineLayoutCreateInfo).setLayoutCount = 0;
    1
}

/// The renderer struct, used for rendering models
pub struct Renderer {
    name_str: CString,

    pipeline_index: u32,

    real_screen_width: u32,
    real_screen_height: u32,
}

impl Renderer {
    #[cfg(target_os = "windows")]
    fn init_vulkan(&self, window: &winit::window::Window) {
        // Using the vulkan helper
        unsafe {
            let res = vh_create_instance_and_surface_win32(
                self.name_str.as_ptr(),
                window.hinstance() as *mut HINSTANCE__,
                window.hwnd() as *mut HWND__,
            );

            if res > 0 {
                println!("Vulkan instance and surface created.")
            } else {
                panic!("Vulkan instance and surface creation has failed.");
            }
        }
    }
    #[cfg(target_os = "linux")]
    fn init_vulkan(&self, window: &winit::window::Window) {
        // Using the vulkan helper

        let c = window.xcb_connection().unwrap();
        let winv: u32 = window.xlib_window().unwrap().try_into().unwrap();

        let w: *mut u32 = &mut winv.clone();

        unsafe {
            let res = vh_create_instance_and_surface_linux(
                self.nameStr.as_ptr(),
                c as *mut xcb_connection_t,
                w,
            );

            if res > 0 {
                println!("Vulkan instance and surface created.")
            } else {
                panic!("Vulkan instance and surface creation has failed.");
            }
        }
    }

    #[cfg(target_os = "macos")]
    fn init_vulkan(&self, window: &winit::window::Window) {
        // Using the vulkan helper
        unsafe {
            let res = vh_create_instance_and_surface_macos(self.nameStr.as_ptr(), window.ns_view());

            if res > 0 {
                println!("Vulkan instance and surface created.")
            } else {
                panic!("Vulkan instance and surface creation has failed.");
            }
        }
    }

    /// Create a renderer instance
    pub fn new(name: &str, window: &winit::window::Window) -> Renderer {
        let mut myself = Self {
            name_str: CString::new(name).expect("CString::new failed"),

            pipeline_index: 100,

            real_screen_width: 1024,
            real_screen_height: 768,
        };

        let work_dir = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let vertex_sharder_path =
            CString::new(work_dir.clone() + "/resources/shaders/vertexShader.spv")
                .expect("CString::new failed");
        let fragment_shader_path = CString::new(work_dir + "/resources/shaders/fragmentShader.spv")
            .expect("CString::new failed");
        unsafe {
            Renderer::init_vulkan(&myself, &window);

            if vh_init(NUM_FRAMES_IN_FLIGHT as u32) != 1 {
                panic!("Could not initialise Vulkan.");
            }

            vh_set_width_height(1024, 768);

            vh_create_sync_objects();

            if vh_create_swapchain() != 1 {
                panic!("Failed to create Vulkan swapchain.");
            }

            let iscb = Option::Some(
                set_input_state_callback
                    as unsafe extern "C" fn(*mut VkPipelineVertexInputStateCreateInfo) -> i32,
            );
            let iscc = Option::Some(
                set_pipeline_layout_callback
                    as unsafe extern "C" fn(*mut VkPipelineLayoutCreateInfo) -> i32,
            );

            let pidx_ptr: *mut u32 = &mut myself.pipeline_index;

            vh_create_pipeline(
                vertex_sharder_path.as_ptr(),
                fragment_shader_path.as_ptr(),
                iscb,
                iscc,
                pidx_ptr,
            );
        }
        myself
    }

    /// Render a model
    pub fn render(&mut self, m: &super::model::Model) {
        let mut current_frame_index = 0;
        let cfi_ptr: *mut u32 = &mut current_frame_index;

        let mut image_index = 0;
        let img_ptr: *mut u32 = &mut image_index;

        unsafe {
            vh_acquire_next_image(self.pipeline_index, img_ptr, cfi_ptr);
            vh_wait_gpu_cpu_fence(current_frame_index);

            let cb_ptr: *mut VkCommandBuffer = &mut command_buffer[current_frame_index as usize];

            vh_destroy_draw_command_buffer(cb_ptr);

            vh_begin_draw_command_buffer(cb_ptr);
            let cb_cptr: *const VkCommandBuffer = &command_buffer[current_frame_index as usize];
            vh_bind_pipeline_to_command_buffer(self.pipeline_index, cb_cptr);
            let binding: VkDeviceSize = 0;
            vkCmdBindVertexBuffers(*cb_ptr, 0, 1, &m.vertex_buffer, &binding);
            vkCmdBindIndexBuffer(
                *cb_ptr,
                m.index_buffer,
                0,
                VkIndexType_VK_INDEX_TYPE_UINT16,
            );
            vkCmdDrawIndexed(*cb_ptr, m.index_data_size, 1, 0, 0, 0);
            vh_end_draw_command_buffer(cb_ptr);

            if vh_draw(cb_ptr, 1) != 1 {
                panic!("vh_draw has failed!");
            }
            if vh_draw(cb_ptr, 0) != 1 {
                panic!("vh_draw has failed!");
            };

            vh_present_next_image();
        }
    }

    /// Shutdown the GPU driver
    pub fn shutdown(&mut self) {
        unsafe {
            vkDeviceWaitIdle(vh_logical_device);
            for idx in 0..NUM_FRAMES_IN_FLIGHT - 1 {
                let cb_p: *mut VkCommandBuffer = &mut command_buffer[idx];
                vh_destroy_draw_command_buffer(cb_p);
            }
            
            vh_destroy_pipeline(self.pipeline_index);

            vh_destroy_swapchain();
            vh_destroy_sync_objects();
            vkDestroySurfaceKHR(vh_instance, vh_surface, std::ptr::null_mut());
            vh_shutdown()
        };
    }

    /// Set the width and length of the rendering window in pixels. This can
    /// be used also during window resize events coming from the application
    /// window.
    pub fn set_width_height(&mut self, width: u32, height: u32) {
        unsafe {
            self.real_screen_width = width;
            self.real_screen_height = height;
            if self.real_screen_width == 0 {
                self.real_screen_width = 1
            };
            if self.real_screen_height == 0 {
                self.real_screen_height = 1
            };
            vh_set_width_height(self.real_screen_width, self.real_screen_height);
            vh_recreate_pipelines_and_swapchain();
        }
    }
}
