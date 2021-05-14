use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use pixels::{Pixels, SurfaceTexture};

use imageproc::drawing::draw_text_mut;
use image::RgbaImage;

use rusttype::{Font, Scale};

use crate::api::APIHandle;

pub struct Overlay {
    event_loop: EventLoop<()>,
    window: Window,
    pixels: Pixels,
}

const FONT_DATA: &[u8] = include_bytes!("./Calibri.ttf");

impl Overlay {
    //Creates a new overlay with a window and event loop
    pub fn new(size: (u32, u32)) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_transparent(true)
            .with_decorations(false)
            .with_always_on_top(true)
            .with_resizable(false)
            .with_inner_size(PhysicalSize::new(size.0, size.1))
            .build(&event_loop)
            .unwrap();

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(window_size.width, window_size.height, surface_texture).unwrap()
        };

        Overlay {
            event_loop: event_loop,
            window,
            pixels,
        }
    }

    //Runs the main loop for the overlay
    pub fn run(self, process_handle: APIHandle) {
        let window = self.window;
        let mut pixels = self.pixels;

        self.event_loop.run(move |event, _, control_flow| {
            match event {
                //Exits the program when required
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,

                //Updates overlay state
                Event::MainEventsCleared => {
                    window.request_redraw();
                }

                Event::RedrawRequested(_) => {
                    let font = Font::try_from_bytes(FONT_DATA).unwrap();

                    let win_size = window.inner_size();
                    let mut canvas = RgbaImage::new(win_size.width, win_size.height);
                    draw_text_mut(&mut canvas, [255, 255, 255, 255].into(), 0, 0, Scale::uniform(40.0), &font, "Test");

                    let frame = pixels.get_frame();
                    frame.copy_from_slice(canvas.as_raw());

                    pixels.render().unwrap();
                }

                _ => (),
            }
        });
    }
}
