use device_query::DeviceQuery;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use pixels::{Pixels, SurfaceTexture};

use image::RgbaImage;
use imageproc::drawing::draw_text_mut;

use rusttype::{Font, Scale};

use crate::api::APIHandle;
use crate::input_handler::InputHandler;

use std::time::{Duration, Instant};

const FONT_DATA: &[u8] = include_bytes!("./Calibri.ttf");

pub struct Overlay {
    event_loop: EventLoop<()>,
    window: Window,
    pixels: Pixels,
    text: String,
    font: Font<'static>,
    input_handler: InputHandler,
}

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

        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(window_size.width, window_size.height, surface_texture).unwrap()
        };

        let font = Font::try_from_bytes(FONT_DATA).unwrap();

        Overlay {
            event_loop: event_loop,
            window,
            pixels,
            text: String::new(),
            font,
            input_handler: InputHandler::new(),
        }
    }

    //Runs the main loop for the overlay
    pub fn run(self, api_handle: APIHandle) {
        let window = self.window;
        let mut pixels = self.pixels;
        let mut text = self.text;
        let font = self.font;
        let mut input_handler = self.input_handler;

        self.event_loop.run(move |event, _, control_flow| {
            match event {
                //Exits the program when required
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,

                //Updates overlay state
                Event::MainEventsCleared => {
                    //Set overlay position
                    let win_rect = api_handle.get_win_rect();
                    let win_size = window.inner_size();

                    let top = win_rect.bottom - win_size.height as i32;
                    window.set_outer_position(PhysicalPosition::new(win_rect.left, top));

                    //Update the focus
                    api_handle.update_focus(&window);

                    input_handler.update(&mut text, &api_handle);

                    window.request_redraw();
                }

                Event::RedrawRequested(_) => {
                    //Draws the text to the canvas
                    let win_size = window.inner_size();
                    let mut canvas = RgbaImage::new(win_size.width, win_size.height);

                    //Manually implements newlines
                    let lines: Vec<&str> = text.split("\n").collect();
                    for i in 0..lines.len() {
                        let y_pos = i * 50;
                        draw_text_mut(
                            &mut canvas,
                            [255, 255, 255, 255].into(),
                            0,
                            y_pos as u32,
                            Scale::uniform(40.0),
                            &font,
                            lines[i],
                        );
                    }

                    //Copies the canvas to the window buffer
                    let frame = pixels.get_frame();
                    frame.copy_from_slice(canvas.as_raw());

                    pixels.render().unwrap();
                }

                _ => (),
            }
        });
    }
}
