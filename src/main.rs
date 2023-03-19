// Don't go crazy with warnings about all the stuff imported from C
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

mod rectangle;

use std::ffi::CString;
use std::os::raw::c_void;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

#[cfg(target_os = "linux")]
use winit::platform::x11::WindowExtX11;

#[cfg(target_os = "windows")]
use winit::platform::windows::WindowExtWindows;

#[cfg(target_os = "macos")]
use winit::platform::macos::WindowExtMacOS;

const NUM_FRAMES_IN_FLIGHT: usize = 3;
const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;

static mut vertexData: [f32; 16] = [0f32; 16];
static mut indexData: [u32; 6] = [0u32; 6];
static mut textureCoordsData: [f32; 8] = [0f32; 8];

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
    (*inputStateCreateInfo).pVertexBindingDescriptions = &binding_desc;
    (*inputStateCreateInfo).pVertexAttributeDescriptions = &attrib_desc;
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

fn main() {

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Clunker")
	.with_visible(true)
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build(&event_loop)
        .unwrap();

    let mut app = unsafe { App::create(&window) };
    let mut destroying = false;

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll(); // vs .set_wait

        match event {
            Event::MainEventsCleared if !destroying => unsafe { app.render() },

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                destroying = true;
                control_flow.set_exit();
                unsafe {
                    app.destroy();
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {}
            _ => (),
        }
    });
}

struct App {
    nameStr: CString,
    pipeline_index: u32,

    vertex_buffer: VkBuffer,
    vertex_buffer_ptr: *mut VkBuffer,

    vertex_buffer_memory: VkDeviceMemory,
    vertex_buffer_memory_ptr: *mut VkDeviceMemory,

    staging_buffer: VkBuffer,
    staging_buffer_ptr: *mut VkBuffer,

    staging_buffer_memory: VkDeviceMemory,
    staging_buffer_memory_ptr: *mut VkDeviceMemory,

    index_buffer: VkBuffer,
    index_buffer_ptr: *mut VkBuffer,
    index_buffer_memory: VkDeviceMemory,
    index_buffer_memory_ptr: *mut VkDeviceMemory,

    xcb_connx: *mut c_void,
    xlib_win: u32,

}

impl App {
    #[cfg(target_os = "windows")]
    unsafe fn initVulkan(&self, window: &Window) {
	// Using the vulkan helper
        let res = vh_create_instance_and_surface_win32(
            self.nameStr.as_ptr(),
            window.hinstance() as *mut HINSTANCE__,
            window.hwnd() as *mut HWND__,
        );

        if res > 0 {
            println!("Vulkan instance and surface created.")
        } else {
            panic!("Vulkan instance and surface creation has failed.");
        }
    }
    #[cfg(target_os = "linux")]
    unsafe fn initVulkan(&self, _window: &Window) {
	// Using the vulkan helper

        let w: *mut u32 = &mut self.xlib_win.clone();

        let res = vh_create_instance_and_surface_linux(
            self.nameStr.as_ptr(),
            self.xcb_connx as *mut xcb_connection_t,
            w,
        );

        if res > 0 {
            println!("Vulkan instance and surface created.")
        } else {
            panic!("Vulkan instance and surface creation has failed.");
        }
    }

    #[cfg(target_os = "macos")]
    unsafe fn initVulkan(&self, window: &Window) {
	// Using the vulkan helper
        let res = vh_create_instance_and_surface_macos(
            self.nameStr.as_ptr(),
            window.ns_view(),
        );

        if res > 0 {
            println!("Vulkan instance and surface created.")
        } else {
            panic!("Vulkan instance and surface creation has failed.");
        }
    }

    unsafe fn create(window: &Window) -> App {
        let mut myself = Self {
            nameStr: CString::new("Hello Rust").expect("CString::new failed"),
            pipeline_index: 100,

            vertex_buffer: std::ptr::null_mut(),
            vertex_buffer_ptr: std::ptr::null_mut(),

            vertex_buffer_memory: std::ptr::null_mut(),
            vertex_buffer_memory_ptr: std::ptr::null_mut(),

            staging_buffer: std::ptr::null_mut(),
            staging_buffer_ptr: std::ptr::null_mut(),

            staging_buffer_memory: std::ptr::null_mut(),
            staging_buffer_memory_ptr: std::ptr::null_mut(),

            index_buffer: std::ptr::null_mut(),
            index_buffer_ptr: std::ptr::null_mut(),
            index_buffer_memory: std::ptr::null_mut(),
            index_buffer_memory_ptr: std::ptr::null_mut(),

            xcb_connx: std::ptr::null_mut(),
            xlib_win: 0u32,
            
        };

        crate::rectangle::create_rectangle(
            -0.5,
            -0.5,
            0.0,
            0.5,
            0.5,
            0.0,
            &mut vertexData,
            &mut indexData,
            &mut textureCoordsData,
        );

        let work_dir = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let vertex_sharder_path =
            CString::new(work_dir.clone() + "/resources/shaders/vertexShader.spv")
                .expect("CString::new failed");
        let fragment_shader_path =
            CString::new(work_dir + "/resources/shaders/fragmentShader.spv")
                .expect("CString::new failed");

        
        myself.xcb_connx = window.xcb_connection().unwrap();
        myself.xlib_win = window.xlib_window().unwrap().try_into().unwrap();

        App::initVulkan(&myself, &window);

        if vh_init(NUM_FRAMES_IN_FLIGHT as u32) != 1 {
            panic!("Could not initialise Vulkan.");
        }

        vh_set_width_height(SCREEN_WIDTH, SCREEN_HEIGHT);

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

        myself.vertex_buffer_ptr = &mut myself.vertex_buffer;
        myself.vertex_buffer_memory_ptr = &mut myself.vertex_buffer_memory;

        if vh_create_buffer(
            myself.vertex_buffer_ptr,
            (VkBufferUsageFlagBits_VK_BUFFER_USAGE_TRANSFER_DST_BIT
                | VkBufferUsageFlagBits_VK_BUFFER_USAGE_VERTEX_BUFFER_BIT)
                .try_into()
                .unwrap(),
            16u32 * (std::mem::size_of::<f32>() as u32),
            myself.vertex_buffer_memory_ptr,
            VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT
                .try_into()
                .unwrap(),
        ) != 1
        {
            panic!("Failed to create postition buffer.");
        }

        myself.staging_buffer_ptr = &mut myself.staging_buffer;
        myself.staging_buffer_memory_ptr = &mut myself.staging_buffer_memory;

        if vh_create_buffer(
            myself.staging_buffer_ptr,
            VkBufferUsageFlagBits_VK_BUFFER_USAGE_TRANSFER_SRC_BIT
                .try_into()
                .unwrap(),
            16u32 * (std::mem::size_of::<f32>() as u32),
            myself.staging_buffer_memory_ptr,
            (VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT
                | VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_HOST_COHERENT_BIT)
                .try_into()
                .unwrap(),
        ) != 1
        {
            panic!("Failed to create staging buffer for vertices.");
        }

        let mut staging_data: *mut ::std::os::raw::c_void = std::ptr::null_mut();
        let mut staging_data_ptr: *mut *mut ::std::os::raw::c_void = &mut staging_data;

        vkMapMemory(
            vh_logical_device,
            myself.staging_buffer_memory,
            0,
            VK_WHOLE_SIZE as u64,
            0,
            staging_data_ptr,
        );

        let src_ptr = &mut vertexData as *const f32;

        std::ptr::copy_nonoverlapping(
            src_ptr as *const u8,
            staging_data as *mut u8,
            16usize * std::mem::size_of::<f32>(),
        );

        vkUnmapMemory(vh_logical_device, myself.staging_buffer_memory);

        vh_copy_buffer(
            myself.staging_buffer,
            myself.vertex_buffer,
            16u64 * (std::mem::size_of::<f32>() as u64),
        );

        vh_destroy_buffer(myself.staging_buffer, myself.staging_buffer_memory);

        myself.index_buffer_ptr = &mut myself.index_buffer;
        myself.index_buffer_memory_ptr = &mut myself.index_buffer_memory;

        if vh_create_buffer(
            myself.index_buffer_ptr,
            (VkBufferUsageFlagBits_VK_BUFFER_USAGE_TRANSFER_DST_BIT
                | VkBufferUsageFlagBits_VK_BUFFER_USAGE_INDEX_BUFFER_BIT)
                .try_into()
                .unwrap(),
            6u32 * (std::mem::size_of::<u32>() as u32),
            myself.index_buffer_memory_ptr,
            VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT
                .try_into()
                .unwrap(),
        ) != 1
        {
            panic!("Failed to create index buffer");
        }

        myself.staging_buffer = std::ptr::null_mut();
        myself.staging_buffer_ptr = &mut myself.staging_buffer;
        myself.staging_buffer_memory = std::ptr::null_mut();
        myself.staging_buffer_memory_ptr = &mut myself.staging_buffer_memory;

        if vh_create_buffer(
            myself.staging_buffer_ptr,
            VkBufferUsageFlagBits_VK_BUFFER_USAGE_TRANSFER_SRC_BIT
                .try_into()
                .unwrap(),
            6u32 * (std::mem::size_of::<u32>() as u32),
            myself.staging_buffer_memory_ptr,
            (VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT
                | VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_HOST_COHERENT_BIT)
                .try_into()
                .unwrap(),
        ) != 1
        {
            panic!("Failed to create staging buffer for indices.");
        }

        staging_data = std::ptr::null_mut();
        staging_data_ptr = &mut staging_data;

        vkMapMemory(
            vh_logical_device,
            myself.staging_buffer_memory,
            0,
            VK_WHOLE_SIZE as u64,
            0,
            staging_data_ptr,
        );

        let src_ptr = &mut indexData as *const u32;

        std::ptr::copy_nonoverlapping(
            src_ptr as *const u8,
            staging_data as *mut u8,
            6usize * std::mem::size_of::<u32>(),
        );

        vkUnmapMemory(vh_logical_device, myself.staging_buffer_memory);

        vh_copy_buffer(
            myself.staging_buffer,
            myself.index_buffer,
            6u64 * (std::mem::size_of::<u32>() as u64),
        );

        vh_destroy_buffer(myself.staging_buffer, myself.staging_buffer_memory);

        myself
    }

    unsafe fn render(&mut self) {
        let mut current_frame_index = 0;
        let cfi_ptr: *mut u32 = &mut current_frame_index;

        let mut image_index = 0;
        let img_ptr: *mut u32 = &mut image_index;


        vh_acquire_next_image(
            self.pipeline_index,
            img_ptr,
            cfi_ptr,
        );
        vh_wait_gpu_cpu_fence(current_frame_index);

        let cb_ptr: *mut VkCommandBuffer = &mut command_buffer[current_frame_index as usize];

        if *cb_ptr == std::ptr::null_mut() {
            vh_begin_draw_command_buffer(cb_ptr);
            let cb_cptr: *const VkCommandBuffer = &command_buffer[current_frame_index as usize];
            vh_bind_pipeline_to_command_buffer(self.pipeline_index, cb_cptr);
            let binding: VkDeviceSize = 0;
            vkCmdBindVertexBuffers(*cb_ptr, 0, 1, &self.vertex_buffer, &binding);
            vkCmdBindIndexBuffer(
                *cb_ptr,
                self.index_buffer,
                0,
                VkIndexType_VK_INDEX_TYPE_UINT32,
            );
            vkCmdDrawIndexed(*cb_ptr, 6, 1, 0, 0, 0);
            vh_end_draw_command_buffer(cb_ptr);
        }

        vh_draw(cb_ptr, 1);
        vh_draw(cb_ptr, 0);
        vh_present_next_image();
    }

    unsafe fn destroy(&mut self) {
        for idx in 0..NUM_FRAMES_IN_FLIGHT - 1 {
            vh_wait_gpu_cpu_fence(idx.try_into().unwrap());
            let cb_p: *mut VkCommandBuffer = &mut command_buffer[idx];
            vh_destroy_draw_command_buffer(cb_p);
        }

        vh_destroy_buffer(self.vertex_buffer, self.vertex_buffer_memory);
        vh_destroy_buffer(self.index_buffer, self.index_buffer_memory);
        vh_destroy_pipeline(self.pipeline_index);

        vh_destroy_swapchain();
        vh_destroy_sync_objects();
        vh_shutdown();
    }
}
