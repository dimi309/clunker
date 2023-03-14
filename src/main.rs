// Don't go crazy with warnings about all the stuff imported from C
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

mod rectangle;

use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    platform::windows::WindowExtWindows,
    window::{Window, WindowBuilder},
};

const NUM_FRAMES_IN_FLIGHT: usize = 3;
const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;


static mut vertexData: [f32; 16] = [0f32; 16];
static mut indexData: [u32; 6] = [0u32; 6];
static mut textureCoordsData: [f32; 8] = [0f32; 8];


static mut binding_desc: VkVertexInputBindingDescription = VkVertexInputBindingDescription {binding: 0, stride: 4u32 * (std::mem::size_of::<f32>() as u32), inputRate: 0};
static mut attrib_desc:  VkVertexInputAttributeDescription = VkVertexInputAttributeDescription {binding: 0, location: 0, format: VkFormat_VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0};
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
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build(&event_loop)
        .unwrap();

    let mut app = unsafe { App::create(&window) };
    let mut destroying = false;

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll(); // vs .set_wait

        match event {
            Event::MainEventsCleared if !destroying => unsafe { app.render(&window) },

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
    pipeline_index: *mut u32,
    image_index: *mut u32,
    
 vertex_buffer: *mut VkBuffer,
 vertex_buffer_memory:  *mut VkDeviceMemory,
 index_buffer: *mut VkBuffer,
index_buffer_memory: *mut VkDeviceMemory  ,
}

impl App {
    unsafe fn create(window: &Window) -> App {
        let myself = Self {
            nameStr: CString::new("Hello Rust").expect("CString::new failed"),
            pipeline_index: &mut 100,
            image_index: std::ptr::null_mut(),
            vertex_buffer: std::ptr::null_mut(),
            vertex_buffer_memory: std::ptr::null_mut(),
            index_buffer: std::ptr::null_mut(),
           index_buffer_memory: std::ptr::null_mut(),

        };

        crate::rectangle::create_rectangle(-0.5, -0.5, 0.0, 0.5, 0.5, 0.0,
            &mut vertexData, &mut indexData, &mut textureCoordsData);

        let work_dir =  std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

        let vertex_sharder_path = CString::new(work_dir.clone() + "\\resources\\shaders\\vertexShader.spv")
            .expect("CString::new failed");
        let fragment_shader_path = CString::new(work_dir + "\\resources\\shaders\\fragmentShader.spv")
            .expect("CString::new failed");

        let mut res: i32 = 0;

        // Using the vulkan helper
        res = vh_create_instance_and_surface_win32(
            myself.nameStr.as_ptr(),
            window.hinstance() as *mut HINSTANCE__,
            window.hwnd() as *mut HWND__,
        );

        if res > 0 {
            println!("Vulkan instance and surface created.")
        } else {
            panic!("Vulkan instance and surface creation has failed.");
        }

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

        vh_create_pipeline(
            vertex_sharder_path.as_ptr(),
            fragment_shader_path.as_ptr(),
            iscb,
            iscc,
            myself.pipeline_index,
        );

        if vh_create_buffer(myself.vertex_buffer, 
            (VkBufferUsageFlagBits_VK_BUFFER_USAGE_TRANSFER_DST_BIT |
            VkBufferUsageFlagBits_VK_BUFFER_USAGE_VERTEX_BUFFER_BIT).try_into().unwrap(),
            16 * <usize as TryInto<f32>>::try_into(std::mem::size_of::<f32>()).unwrap(),
            myself.vertex_buffer_memory,
            VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT.try_into().unwrap()) != 1 {
            panic!("Failed to create postition buffer");
          }

        myself
    }

    unsafe fn render(&mut self, window: &Window) {}

    unsafe fn destroy(&mut self) {
        vh_destroy_swapchain();
        vh_destroy_sync_objects();
        vh_shutdown();
    }
}
