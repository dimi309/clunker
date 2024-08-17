mod model;
mod renderer;

use std::io::BufReader;
use std::fs::File;

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;

use rodio::{Decoder, OutputStream, source::Source};

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};


fn main() {

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("bah.ogg").unwrap());
    let source = Decoder::new(file).unwrap();
    let _ = stream_handle.play_raw(source.convert_samples());

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Clunker")
        .with_visible(true)
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build(&event_loop)
        .unwrap();
    
    #[cfg(not(debug_assertions))]
    window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(window.current_monitor())));

    let mut app = App::create(&window);

    app.renderer.set_width_height(SCREEN_WIDTH, SCREEN_HEIGHT);


    let mut m = model::Model {

        buffers: Vec::<gltf::buffer::Data>::new(),
        vertex_data: Vec::<f32>::new(),
        index_data: Vec::<u16>::new(),
        normals_data: Vec::<f32>::new(),

        vertex_buffer: std::ptr::null_mut(),
        vertex_buffer_memory: std::ptr::null_mut(),
        index_buffer: std::ptr::null_mut(),
        index_buffer_memory: std::ptr::null_mut(),
        index_data_size: 0,

    };

    m.load("goat.glb");

    app.renderer.to_gpu(&mut m);


    let mut destroying = false;

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll(); // vs .set_wait

        match event {
            Event::MainEventsCleared if !destroying => app.renderer.render(&m),

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                .. // ignore WindowId
            } => {
                destroying = true;
                control_flow.set_exit();
                
                unsafe {
                  renderer::vh_destroy_buffer(m.vertex_buffer, m.vertex_buffer_memory);
                  renderer::vh_destroy_buffer(m.index_buffer, m.index_buffer_memory);
                }

                app.renderer.destroy();

                


            }
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                .. // ignore WindowId
                
            } => {
                app.renderer.set_width_height(new_size.width, new_size.height);
            }

            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => (),
            _ => (),
        }
    });
}

struct App {
    
    renderer: renderer::Renderer,
}

impl App {

    fn create(window: &Window) -> App {
        let myself = Self {
            renderer: renderer::Renderer::create("Clunker", window),
        };
        
        myself
    }
    
}
