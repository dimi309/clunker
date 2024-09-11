mod model;
mod renderer;
mod descriptor;

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

    let mut m = model::Model::new();

    m.load("goat.glb");

    m.to_gpu();

    let mut destroying = false;

    let position = [0.0f32, -0.2f32, 0.0f32];

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll(); // vs .set_wait

        match event {
            Event::MainEventsCleared if !destroying => app.renderer.render(&m, position),

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                .. // ignore WindowId
            } => {
                destroying = true;
                control_flow.set_exit();
                
                m.clear_gpu();

                app.renderer.shutdown();

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
            renderer: renderer::Renderer::new("Clunker", window),
        };
        
        myself
    }
    
}
