// Don't go crazy with warnings about all the stuff imported from C
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

const NUM_FRAMES_IN_FLIGHT: u32 = 3;
const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder}, platform::windows::WindowExtWindows,
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



fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Clunker")
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH,SCREEN_HEIGHT))
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
                + "\\resources\\shaders\\fragmentShader.spv",
        )
        .expect("CString::new failed");
        let mut res: i32 = 0;
        
        // Using the vulkan helper
        res = vh_create_instance_and_surface_win32(myself.nameStr.as_ptr(), window.hinstance() as *mut HINSTANCE__, window.hwnd() as *mut HWND__);

        if res > 0 {
            println!("Vulkan instance and surface created.")
        }
        else {
            panic!("Vulkan instance and surface creation has failed.");
        }

        if vh_init(NUM_FRAMES_IN_FLIGHT) != 1 {
            panic!("Could not initialise Vulkan.");
        }

        vh_set_width_height(SCREEN_WIDTH, SCREEN_HEIGHT);

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

        
        myself
    }

    unsafe fn render(&mut self, window: &Window) {}

    unsafe fn destroy(&mut self) {
        vh_shutdown();
    }
}
