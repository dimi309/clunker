// Don't go crazy with warnings about all the stuff imported from C
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

use std::ffi::CString;
use std::os::raw::c_char;
const ARRAY_SIZE: usize = 30;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

unsafe extern "C" fn set_input_state_callback(
    insputStateCreateInfo: *mut VkPipelineVertexInputStateCreateInfo,
) -> i32 {
    println!("Input state callback called.");
    1
}

unsafe extern "C" fn set_pipeline_layout_callback(
    pipelineLayoutCreateInfo: *mut VkPipelineLayoutCreateInfo,
) -> i32 {
    println!("Pipeline layout callback called.");
    1
}

unsafe fn to_c_str(byte_sequence: &[u8], array: &mut [i8; ARRAY_SIZE])  {
    let mut index = 0;
    while index < byte_sequence.len() && index + 1 < ARRAY_SIZE {
        if byte_sequence[index] != 0 {
            array[index] = byte_sequence[index] as c_char;
            index += 1;
        } else {
            break;
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Clunker")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)
        .unwrap();

    let mut app = unsafe { App::create(&window) };
    let mut destroying = false;

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll(); // vs .set_wait

        match event {

            Event::MainEventsCleared if !destroying =>
            unsafe {
                app.render(&window)
            }

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
}

impl App {
    unsafe fn create(window: &Window) -> App {
        let myself = Self {
            nameStr: CString::new("Hello Rust").expect("CString::new failed"),
            pipeline_index: &mut 100,
        };
        let vertex_sharder_path = CString::new(
            std::env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
                + "\\resources\\shaders\\vertexShader.spv",
        )
        .expect("CString::new failed");
        let fragment_shader_path = CString::new(
            std::env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
                + "resources\\shaders\\vertexShader.spv",
        )
        .expect("CString::new failed");
        let mut res: i32 = 0;

       

        let mut array = [0i8; ARRAY_SIZE];

        to_c_str(VK_KHR_SURFACE_EXTENSION_NAME, &mut array);

        let b = [array.as_ptr()].as_mut_ptr();

        // Using the vulkan helper
        res = vh_create_instance(myself.nameStr.as_ptr(), b, 1);

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

        if res > 0 {
            println!("Vulkan worked!")
        }
        myself
    }

    unsafe fn render(&mut self, window: &Window) {}

    unsafe fn destroy(&mut self) {
        vh_shutdown();
    }
}
